use crate::error::{Error, Result};
use crate::xdr;
use base64;

bitflags! {
    pub struct AccountFlags: u32 {
        const AUTH_REQUIRED = xdr::AccountFlags::AuthRequiredFlag as u32;
        const AUTH_REVOCABLE = xdr::AccountFlags::AuthRevocableFlag as u32;
        const AUTH_IMMUTABLE = xdr::AccountFlags::AuthImmutableFlag as u32;
    }
}

bitflags! {
    pub struct TrustLineFlags: u32 {
        const AUTHORIZED = xdr::TrustLineFlags::AuthorizedFlag as u32;
        const AUTHORIZED_TO_MAINTAIN_LIABILITIES = xdr::TrustLineFlags::AuthorizedToMaintainLiabilitiesFlag as u32;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataValue(Vec<u8>);

impl DataValue {
    pub fn from_slice(value: &[u8]) -> Result<DataValue> {
        if value.len() > 64 {
            return Err(Error::InvalidDataValue);
        }
        Ok(DataValue(value.to_vec()))
    }

    pub fn from_base64(encoded: &str) -> Result<DataValue> {
        let decoded = base64::decode(encoded)?;
        DataValue::from_slice(&decoded)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn to_xdr(&self) -> Result<xdr::DataValue> {
        let inner = self.as_bytes().to_vec();
        Ok(xdr::DataValue::new(inner))
    }

    pub fn from_xdr(x: &xdr::DataValue) -> Result<DataValue> {
        DataValue::from_slice(&x.value)
    }
}
