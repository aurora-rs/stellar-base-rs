//! Traits for XDR serialization and deserialization.
use crate::error::Result;
pub use crate::xdr_generated::*;

pub trait XDRSerialize {
    fn write_xdr(&self, out: &mut Vec<u8>) -> Result<u64>;

    fn xdr_bytes(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        self.write_xdr(&mut out)?;
        Ok(out)
    }

    fn xdr_base64(&self) -> Result<String> {
        let bytes = self.xdr_bytes()?;
        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes);
        Ok(encoded)
    }
}

pub trait XDRDeserialize: Sized {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)>;
    fn from_xdr_base64(encoded: &str) -> Result<Self> {
        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)?;
        let (res, _) = Self::from_xdr_bytes(&decoded)?;
        Ok(res)
    }
}
