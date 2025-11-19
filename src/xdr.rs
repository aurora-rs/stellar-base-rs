//! Traits for XDR serialization and deserialization.

// Re-export types from stellar-xdr::curr for compatibility
pub use stellar_xdr::curr::*;

pub type Result<T> = std::result::Result<T, Error>;

/// Provided for compatibility. Prefer using the `WriteXdr` trait with appropriate limits.
pub trait XDRSerialize {
    fn write_xdr(&self, out: &mut Vec<u8>) -> crate::error::Result<u64>;

    fn xdr_bytes(&self) -> crate::error::Result<Vec<u8>> {
        let mut out = Vec::new();
        self.write_xdr(&mut out)?;
        Ok(out)
    }

    fn xdr_base64(&self) -> crate::error::Result<String> {
        let bytes = self.xdr_bytes()?;
        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes);
        Ok(encoded)
    }
}

impl<T: WriteXdr + ?Sized> XDRSerialize for T {
    fn write_xdr(&self, out: &mut Vec<u8>) -> crate::error::Result<u64> {
        use std::io::Cursor;
        let mut w = Limited::new(Cursor::new(out), Limits::none());
        self.write_xdr(&mut w)
            .map_err(|_| crate::error::Error::XdrError)?;
        let bytes_written = w.inner.position();
        Ok(bytes_written)
    }
}

/// Provided for compatibility. Prefer using the `ReadXdr` trait with appropriate limits.
pub trait XDRDeserialize: Sized {
    fn from_xdr_bytes(buffer: &[u8]) -> crate::error::Result<(Self, u64)>;
    fn from_xdr_base64(encoded: &str) -> crate::error::Result<Self> {
        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)?;
        let (res, _) = Self::from_xdr_bytes(&decoded)?;
        Ok(res)
    }
}

impl<T: ReadXdr + Sized> XDRDeserialize for T {
    fn from_xdr_bytes(buffer: &[u8]) -> crate::error::Result<(Self, u64)> {
        use std::io::Cursor;
        let mut r = Limited::new(Cursor::new(buffer), Limits::none());
        let val = T::read_xdr(&mut r).map_err(|_| crate::error::Error::XdrError)?;
        let bytes_read = r.inner.position();
        Ok((val, bytes_read))
    }
}
