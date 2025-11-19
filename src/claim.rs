//! Represent an account claim.
use std::io::{Read, Write};

use crate::crypto::PublicKey;
use crate::error::{Error, Result};
use crate::xdr;
use chrono::{DateTime, Duration, TimeZone, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimableBalanceId(Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claimant {
    destination: PublicKey,
    predicate: ClaimPredicate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimPredicate {
    Unconditional,
    And(Box<ClaimPredicate>, Box<ClaimPredicate>),
    Or(Box<ClaimPredicate>, Box<ClaimPredicate>),
    Not(Box<ClaimPredicate>),
    BeforeAbsoluteTime(DateTime<Utc>),
    BeforeRelativeTime(Duration),
}

impl ClaimableBalanceId {
    /// Returns a new claimable balance id, or Error if the hash length is not 32 bytes.
    pub fn new(hash: Vec<u8>) -> Result<ClaimableBalanceId> {
        if hash.len() != 32 {
            Err(Error::InvalidClaimableBalanceIdLength)
        } else {
            Ok(ClaimableBalanceId(hash))
        }
    }

    /// Retrieves the claimable balance id bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> xdr::ClaimableBalanceId {
        let hash = xdr::Hash(self.0.as_slice().try_into().unwrap());
        xdr::ClaimableBalanceId::ClaimableBalanceIdTypeV0(hash)
    }

    /// Creates from the xdr object.
    pub fn from_xdr(x: &xdr::ClaimableBalanceId) -> Result<ClaimableBalanceId> {
        match x {
            xdr::ClaimableBalanceId::ClaimableBalanceIdTypeV0(hash) => {
                ClaimableBalanceId::new(hash.0.to_vec())
            }
        }
    }
}

impl Claimant {
    /// Returns a new claimant with the given `destination` and `predicate`.
    pub fn new(destination: PublicKey, predicate: ClaimPredicate) -> Claimant {
        Claimant {
            destination,
            predicate,
        }
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::Claimant> {
        let destination = self.destination.to_xdr_account_id()?;
        let predicate = self.predicate.to_xdr()?;
        let inner = xdr::ClaimantV0 {
            destination,
            predicate,
        };
        Ok(xdr::Claimant::ClaimantTypeV0(inner))
    }

    /// Creates from the xdr object.
    pub fn from_xdr(x: &xdr::Claimant) -> Result<Claimant> {
        match x {
            xdr::Claimant::ClaimantTypeV0(inner) => {
                let destination = PublicKey::from_xdr_account_id(&inner.destination)?;
                let predicate = ClaimPredicate::from_xdr(&inner.predicate)?;
                Ok(Claimant::new(destination, predicate))
            }
        }
    }
}

impl ClaimPredicate {
    /// Returns an unconditional predicate.
    pub fn new_unconditional() -> ClaimPredicate {
        ClaimPredicate::Unconditional
    }

    /// Returns a predicate that is true if `p1` and `p2` are both true.
    pub fn new_and(p1: ClaimPredicate, p2: ClaimPredicate) -> ClaimPredicate {
        ClaimPredicate::And(Box::new(p1), Box::new(p2))
    }

    /// Returns a predicate that is true if at least one of `p1` or `p2` is true.
    pub fn new_or(p1: ClaimPredicate, p2: ClaimPredicate) -> ClaimPredicate {
        ClaimPredicate::Or(Box::new(p1), Box::new(p2))
    }

    /// Returns a predicate that is true if `predicate` is false.
    pub fn new_not(predicate: ClaimPredicate) -> ClaimPredicate {
        ClaimPredicate::Not(Box::new(predicate))
    }

    /// Returns a predicate that is true the ledger close time is before `datetime`.
    pub fn new_before_absolute_time(datetime: DateTime<Utc>) -> ClaimPredicate {
        ClaimPredicate::BeforeAbsoluteTime(datetime)
    }

    /// Returns a predicate that is true the ledger close time is
    /// within `duration` of the current ledger close time.
    pub fn new_before_relative_time(duration: Duration) -> ClaimPredicate {
        ClaimPredicate::BeforeRelativeTime(duration)
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::ClaimPredicate> {
        match self {
            ClaimPredicate::Unconditional => Ok(xdr::ClaimPredicate::Unconditional),
            ClaimPredicate::And(p1, p2) => {
                let p1_xdr = p1.to_xdr()?;
                let p2_xdr = p2.to_xdr()?;
                let predicates = vec![p1_xdr, p2_xdr];
                Ok(xdr::ClaimPredicate::And(predicates.try_into().unwrap()))
            }
            ClaimPredicate::Or(p1, p2) => {
                let p1_xdr = p1.to_xdr()?;
                let p2_xdr = p2.to_xdr()?;
                let predicates = vec![p1_xdr, p2_xdr];
                Ok(xdr::ClaimPredicate::Or(predicates.try_into().unwrap()))
            }
            ClaimPredicate::Not(p) => {
                let p_xdr = p.to_xdr()?;
                let predicate = Some(Box::new(p_xdr));
                Ok(xdr::ClaimPredicate::Not(predicate))
            }
            ClaimPredicate::BeforeAbsoluteTime(datetime) => {
                let time = datetime.timestamp();
                Ok(xdr::ClaimPredicate::BeforeAbsoluteTime(time))
            }
            ClaimPredicate::BeforeRelativeTime(duration) => {
                let time = duration.num_seconds();
                Ok(xdr::ClaimPredicate::BeforeRelativeTime(time))
            }
        }
    }

    /// Creates from the xdr object.
    pub fn from_xdr(x: &xdr::ClaimPredicate) -> Result<ClaimPredicate> {
        // The challenge here is that the XDR definition allows and/or/not
        // predicates to have the wrong number of arguments (because xdrc
        // does not support recursive data structures).
        // We perform a check and return an error if the XDR is valid
        // but the claim predicate is not.
        match x {
            xdr::ClaimPredicate::Unconditional => Ok(ClaimPredicate::new_unconditional()),
            xdr::ClaimPredicate::And(predicates) => {
                let mut p = predicates.iter();
                match (p.next(), p.next()) {
                    (Some(p1), Some(p2)) => {
                        let p1 = ClaimPredicate::from_xdr(p1)?;
                        let p2 = ClaimPredicate::from_xdr(p2)?;
                        Ok(ClaimPredicate::new_and(p1, p2))
                    }
                    _ => Err(Error::XdrClaimPredicateError),
                }
            }
            xdr::ClaimPredicate::Or(predicates) => {
                let mut p = predicates.iter();
                match (p.next(), p.next()) {
                    (Some(p1), Some(p2)) => {
                        let p1 = ClaimPredicate::from_xdr(p1)?;
                        let p2 = ClaimPredicate::from_xdr(p2)?;
                        Ok(ClaimPredicate::new_or(p1, p2))
                    }
                    _ => Err(Error::XdrClaimPredicateError),
                }
            }
            xdr::ClaimPredicate::Not(predicate) => {
                if let Some(predicate) = predicate {
                    let p = ClaimPredicate::from_xdr(predicate)?;
                    Ok(ClaimPredicate::new_not(p))
                } else {
                    Err(Error::XdrClaimPredicateError)
                }
            }
            xdr::ClaimPredicate::BeforeAbsoluteTime(time) => {
                let datetime = Utc.timestamp_opt(*time, 0).single();
                datetime
                    .map(ClaimPredicate::new_before_absolute_time)
                    .ok_or(Error::XdrClaimPredicateError)
            }
            xdr::ClaimPredicate::BeforeRelativeTime(time) => {
                let duration = Duration::seconds(*time);
                Ok(ClaimPredicate::new_before_relative_time(duration))
            }
        }
    }
}

impl xdr::WriteXdr for Claimant {
    fn write_xdr<W: Write>(&self, w: &mut xdr::Limited<W>) -> xdr::Result<()> {
        let xdr = self.to_xdr().map_err(|_| xdr::Error::Invalid)?;
        xdr.write_xdr(w)
    }
}

impl xdr::ReadXdr for Claimant {
    fn read_xdr<R: Read>(r: &mut xdr::Limited<R>) -> xdr::Result<Self> {
        let xdr_result = xdr::Claimant::read_xdr(r)?;
        Self::from_xdr(&xdr_result).map_err(|_| xdr::Error::Invalid)
    }
}

impl xdr::WriteXdr for ClaimPredicate {
    fn write_xdr<W: Write>(&self, w: &mut xdr::Limited<W>) -> xdr::Result<()> {
        let xdr = self.to_xdr().map_err(|_| xdr::Error::Invalid)?;
        xdr.write_xdr(w)
    }
}

impl xdr::ReadXdr for ClaimPredicate {
    fn read_xdr<R: Read>(r: &mut xdr::Limited<R>) -> xdr::Result<Self> {
        let xdr_result = xdr::ClaimPredicate::read_xdr(r)?;
        Self::from_xdr(&xdr_result).map_err(|_| xdr::Error::Invalid)
    }
}
