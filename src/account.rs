//! Account data and flags.
use crate::error::{Error, Result};
use crate::xdr;

bitflags! {
    /// Account flags.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct AccountFlags: u32 {
        const AUTH_REQUIRED = xdr::AccountFlags::RequiredFlag as u32;
        const AUTH_REVOCABLE = xdr::AccountFlags::RevocableFlag as u32;
        const AUTH_IMMUTABLE = xdr::AccountFlags::ImmutableFlag as u32;
        const AUTH_CLAWBACK_ENABLED = xdr::AccountFlags::ClawbackEnabledFlag as u32;
    }
}

bitflags! {
    /// Account trust line flags.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct TrustLineFlags: u32 {
        const AUTHORIZED = xdr::TrustLineFlags::AuthorizedFlag as u32;
        const AUTHORIZED_TO_MAINTAIN_LIABILITIES = xdr::TrustLineFlags::AuthorizedToMaintainLiabilitiesFlag as u32;
        const TRUSTLINE_CLAWBACK_ENABLED = xdr::TrustLineFlags::TrustlineClawbackEnabledFlag as u32;
    }
}

/// Data associated with a Stellar account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataValue(Vec<u8>);

impl DataValue {
    /// Converts a slice of bytes to a DataValue.
    ///
    /// Returns Err if the slice is longer than 64 bytes.
    pub fn from_slice(value: &[u8]) -> Result<DataValue> {
        if value.len() > 64 {
            return Err(Error::InvalidDataValue);
        }
        Ok(DataValue(value.to_vec()))
    }

    /// Converts bytes encoded as base64 to a DataValue.
    ///
    /// Returns Err if the encoded data is longer than 64 bytes.
    pub fn from_base64(encoded: &str) -> Result<DataValue> {
        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)?;
        DataValue::from_slice(&decoded)
    }

    /// Returns the DataValue content as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the DataValue xdr object.
    pub fn to_xdr(&self) -> Result<xdr::DataValue> {
        let inner = self.as_bytes();
        Ok(xdr::DataValue(
            inner.try_into().map_err(|_| Error::XdrError)?,
        ))
    }

    /// Creates a DataValue from xdr object.
    pub fn from_xdr(x: &xdr::DataValue) -> Result<DataValue> {
        DataValue::from_slice(&x)
    }
}
