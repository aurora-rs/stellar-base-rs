//! Represent an account claim.
use crate::crypto::PublicKey;
use crate::error::{Error, Result};
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use chrono::{DateTime, Duration, TimeZone, Utc};
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

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
            ClaimPredicate::Unconditional => {
                Ok(xdr::ClaimPredicate::ClaimPredicateUnconditional(()))
            }
            ClaimPredicate::And(p1, p2) => {
                let p1_xdr = p1.to_xdr()?;
                let p2_xdr = p2.to_xdr()?;
                let predicates = vec![Box::new(p1_xdr), Box::new(p2_xdr)];
                Ok(xdr::ClaimPredicate::ClaimPredicateAnd(predicates))
            }
            ClaimPredicate::Or(p1, p2) => {
                let p1_xdr = p1.to_xdr()?;
                let p2_xdr = p2.to_xdr()?;
                let predicates = vec![Box::new(p1_xdr), Box::new(p2_xdr)];
                Ok(xdr::ClaimPredicate::ClaimPredicateOr(predicates))
            }
            ClaimPredicate::Not(p) => {
                let p_xdr = p.to_xdr()?;
                let predicate = Some(Box::new(p_xdr));
                Ok(xdr::ClaimPredicate::ClaimPredicateNot(predicate))
            }
            ClaimPredicate::BeforeAbsoluteTime(datetime) => {
                let time = xdr::Int64::new(datetime.timestamp());
                Ok(xdr::ClaimPredicate::ClaimPredicateBeforeAbsoluteTime(time))
            }
            ClaimPredicate::BeforeRelativeTime(duration) => {
                let time = xdr::Int64::new(duration.num_seconds());
                Ok(xdr::ClaimPredicate::ClaimPredicateBeforeRelativeTime(time))
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
            xdr::ClaimPredicate::ClaimPredicateUnconditional(()) => {
                Ok(ClaimPredicate::new_unconditional())
            }
            xdr::ClaimPredicate::ClaimPredicateAnd(predicates) => {
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
            xdr::ClaimPredicate::ClaimPredicateOr(predicates) => {
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
            xdr::ClaimPredicate::ClaimPredicateNot(predicate) => {
                if let Some(predicate) = predicate {
                    let p = ClaimPredicate::from_xdr(predicate)?;
                    Ok(ClaimPredicate::new_not(p))
                } else {
                    Err(Error::XdrClaimPredicateError)
                }
            }
            xdr::ClaimPredicate::ClaimPredicateBeforeAbsoluteTime(time) => {
                let datetime = Utc.timestamp(time.value, 0);
                Ok(ClaimPredicate::new_before_absolute_time(datetime))
            }
            xdr::ClaimPredicate::ClaimPredicateBeforeRelativeTime(time) => {
                let duration = Duration::seconds(time.value);
                Ok(ClaimPredicate::new_before_relative_time(duration))
            }
        }
    }
}

impl XDRSerialize for Claimant {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr = self.to_xdr()?;
        xdr.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for Claimant {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_claimant, bytes_read) =
            xdr::Claimant::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = Claimant::from_xdr(&xdr_claimant)?;
        Ok((res, bytes_read))
    }
}

impl XDRSerialize for ClaimPredicate {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr = self.to_xdr()?;
        xdr.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for ClaimPredicate {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_claim, bytes_read) =
            xdr::ClaimPredicate::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = ClaimPredicate::from_xdr(&xdr_claim)?;
        Ok((res, bytes_read))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::Amount;
    use crate::asset::Asset;
    use crate::claim::{ClaimPredicate, Claimant};
    use crate::crypto::KeyPair;
    use crate::network::Network;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use chrono::Duration;
    use std::str::FromStr;

    fn keypair0() -> KeyPair {
        // GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3
        KeyPair::from_secret_seed("SBPQUZ6G4FZNWFHKUWC5BEYWF6R52E3SEP7R3GWYSM2XTKGF5LNTWW4R")
            .unwrap()
    }

    fn keypair1() -> KeyPair {
        // GAS4V4O2B7DW5T7IQRPEEVCRXMDZESKISR7DVIGKZQYYV3OSQ5SH5LVP
        KeyPair::from_secret_seed("SBMSVD4KKELKGZXHBUQTIROWUAPQASDX7KEJITARP4VMZ6KLUHOGPTYW")
            .unwrap()
    }

    fn keypair2() -> KeyPair {
        // GB7BDSZU2Y27LYNLALKKALB52WS2IZWYBDGY6EQBLEED3TJOCVMZRH7H
        KeyPair::from_secret_seed("SBZVMB74Z76QZ3ZOY7UTDFYKMEGKW5XFJEB6PFKBF4UYSSWHG4EDH7PY")
            .unwrap()
    }

    #[test]
    fn test_create_claimable_balance() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();
        let dest = kp1.public_key();

        let amount = Amount::from_str("12.0333").unwrap();
        let asset = Asset::new_credit("ABCD", kp2.public_key().clone()).unwrap();

        let predicate =
            ClaimPredicate::new_not(ClaimPredicate::new_before_relative_time(Duration::days(7)));

        let claimant = Claimant::new(kp1.public_key().clone(), predicate);

        let op = Operation::new_create_claimable_balance()
            .with_asset(asset)
            .with_amount(amount)
            .unwrap()
            .add_claimant(claimant)
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAOAAAAAUFCQ0QAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAABywiyAAAAAEAAAAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAwAAAAEAAAAFAAAAAAAJOoAAAAAAAAAAAeoucsUAAABAUA3iWSLubKZc6r4CL2s9WTr/xMS5zuWgzxvm2hBs9use/2ejCagSPlRBeRCe3Ky4R+tKMk8Qpa2LATvgUQS2BQ==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_create_claimable_balance_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();
        let dest = kp1.public_key();

        let amount = Amount::from_str("12.0333").unwrap();
        let asset = Asset::new_credit("ABCD", kp2.public_key().clone()).unwrap();

        let predicate =
            ClaimPredicate::new_not(ClaimPredicate::new_before_relative_time(Duration::days(7)));

        let claimant = Claimant::new(kp1.public_key().clone(), predicate);

        let op = Operation::new_create_claimable_balance()
            .with_source_account(kp.public_key().clone())
            .with_asset(asset)
            .with_amount(amount)
            .unwrap()
            .add_claimant(claimant)
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAA4Nxt4XJcrGZRYrUvrOc1sooiQ+QdEk1suS1wo+oucsUAAAAOAAAAAUFCQ0QAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAABywiyAAAAAEAAAAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAwAAAAEAAAAFAAAAAAAJOoAAAAAAAAAAAeoucsUAAABAcaaQuqZMwpwVMS9814lZPhjt43B3xwlGNfeyx2wU2EJSDJ0h0d2a7dxngMzq4/abNVCjBKspCU7XroelAhSNCw==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
