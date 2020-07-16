//! Assets on the network.
use crate::crypto::PublicKey;
use crate::error::{Error, Result};
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

/// Represent an asset, either the native asset (XLM) or an asset
/// issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asset {
    /// The native asset (XLM).
    Native,
    /// A non-native asset, identified by asset code/issuer id.
    Credit(CreditAsset),
}

/// The credit asset type, based on its code length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreditAssetType {
    CreditAlphaNum4(String),
    CreditAlphaNum12(String),
}

/// A non-native asset, identified by asset code/issuer id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreditAsset {
    AlphaNum4 { code: String, issuer: PublicKey },
    AlphaNum12 { code: String, issuer: PublicKey },
}

impl Asset {
    /// Create the native asset: Lumens.
    pub fn new_native() -> Asset {
        Asset::Native
    }

    /// Create the asset with `code` issued by `issuer`.
    pub fn new_credit<S>(code: S, issuer: PublicKey) -> Result<Asset>
    where
        S: Into<String>,
    {
        let code = code.into();
        let inner = CreditAsset::new(code, issuer)?;
        Ok(Asset::Credit(inner))
    }

    /// Returns true if the asset is a Native. Returns false otherwise.
    pub fn is_native(&self) -> bool {
        match *self {
            Asset::Native => true,
            _ => false,
        }
    }

    /// If the asset is a Credit, returns its value. Returns None otherwise
    pub fn as_credit(&self) -> Option<&CreditAsset> {
        match *self {
            Asset::Credit(ref credit) => Some(credit),
            _ => None,
        }
    }

    /// If the asset is a Credit, returns its mutable value. Returns None otherwise
    pub fn as_credit_mut(&mut self) -> Option<&mut CreditAsset> {
        match *self {
            Asset::Credit(ref mut credit) => Some(credit),
            _ => None,
        }
    }

    /// Returns true if the asset is a Credit. Returns false otherwise.
    pub fn is_credit(&self) -> bool {
        self.as_credit().is_some()
    }

    /// Returns the asset xdr object.
    pub fn to_xdr(&self) -> Result<xdr::Asset> {
        match self {
            Asset::Native => Ok(xdr::Asset::AssetTypeNative(())),
            Asset::Credit(credit) => match credit {
                CreditAsset::AlphaNum4 { code, issuer } => {
                    let code_len = code.len();
                    let mut code_bytes = Vec::with_capacity(4);
                    code_bytes.resize(4, 0);
                    code_bytes[..code_len].copy_from_slice(code.as_bytes());
                    let asset_code = xdr::AssetCode4::new(code_bytes);
                    let issuer = issuer.to_xdr_account_id()?;
                    let asset_alphanum4 = xdr::AssetAlphaNum4 { asset_code, issuer };
                    Ok(xdr::Asset::AssetTypeCreditAlphanum4(asset_alphanum4))
                }
                CreditAsset::AlphaNum12 { code, issuer } => {
                    let code_len = code.len();
                    let mut code_bytes = Vec::with_capacity(12);
                    code_bytes.resize(12, 0);
                    code_bytes[..code_len].copy_from_slice(code.as_bytes());
                    let asset_code = xdr::AssetCode12::new(code_bytes);
                    let issuer = issuer.to_xdr_account_id()?;
                    let asset_alphanum12 = xdr::AssetAlphaNum12 { asset_code, issuer };
                    Ok(xdr::Asset::AssetTypeCreditAlphanum12(asset_alphanum12))
                }
            },
        }
    }

    /// Creates an asset from the xdr object.
    pub fn from_xdr(x: &xdr::Asset) -> Result<Asset> {
        match x {
            xdr::Asset::AssetTypeNative(()) => Ok(Asset::new_native()),
            xdr::Asset::AssetTypeCreditAlphanum4(credit) => {
                let issuer = PublicKey::from_xdr_account_id(&credit.issuer)?;
                let code = xdr_code_to_string(&credit.asset_code.value);
                Asset::new_credit(code, issuer)
            }
            xdr::Asset::AssetTypeCreditAlphanum12(credit) => {
                let issuer = PublicKey::from_xdr_account_id(&credit.issuer)?;
                let code = xdr_code_to_string(&credit.asset_code.value);
                Asset::new_credit(code, issuer)
            }
        }
    }
}

impl CreditAsset {
    /// Creates new credit asset with `code` and `issuer`.
    ///
    /// Code must be shorter than 12 characters.
    pub fn new(code: String, issuer: PublicKey) -> Result<CreditAsset> {
        let code_len = code.len();
        if code_len >= 1 && code_len <= 4 {
            Ok(CreditAsset::AlphaNum4 { code, issuer })
        } else if code_len >= 5 && code_len <= 12 {
            Ok(CreditAsset::AlphaNum12 { code, issuer })
        } else {
            Err(Error::InvalidAssetCode)
        }
    }

    /// Returns the asset code.
    pub fn code(&self) -> &str {
        match self {
            CreditAsset::AlphaNum4 { code, issuer: _ } => &code,
            CreditAsset::AlphaNum12 { code, issuer: _ } => &code,
        }
    }

    /// Returns the asset issuer.
    pub fn issuer(&self) -> &PublicKey {
        match self {
            CreditAsset::AlphaNum4 { code: _, issuer } => &issuer,
            CreditAsset::AlphaNum12 { code: _, issuer } => &issuer,
        }
    }

    /// Returns the credit asset type.
    pub fn asset_type(&self) -> CreditAssetType {
        match self {
            CreditAsset::AlphaNum4 { code, issuer: _ } => {
                CreditAssetType::CreditAlphaNum4(code.clone())
            }
            CreditAsset::AlphaNum12 { code, issuer: _ } => {
                CreditAssetType::CreditAlphaNum12(code.clone())
            }
        }
    }
}

impl XDRSerialize for Asset {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr_asset = self.to_xdr()?;
        xdr_asset.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for Asset {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_asset, bytes_read) = xdr::Asset::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = Asset::from_xdr(&xdr_asset)?;
        Ok((res, bytes_read))
    }
}

/// Create new String from asset code. Make sure not to copy zero bytes.
pub(crate) fn xdr_code_to_string(x: &[u8]) -> String {
    let mut pos = 0;
    for i in 0..x.len() {
        if x[i] == 0 {
            break;
        }
        pos += 1;
    }
    String::from_utf8_lossy(&x[..pos]).into_owned()
}

#[cfg(test)]
mod tests {
    use super::{Asset, CreditAsset};
    use crate::crypto::PublicKey;
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_error_code_too_long() {
        let code = "1234567890123".to_string();
        let pk =
            PublicKey::from_account_id("GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D")
                .unwrap();
        let asset = CreditAsset::new(code, pk);
        assert!(asset.is_err());
    }

    #[test]
    fn test_asset_native_xdr_ser() {
        let native = Asset::new_native();
        let xdr = native.xdr_base64().unwrap();
        assert_eq!("AAAAAA==", xdr);
    }

    #[test]
    fn test_asset_alphanum4_xdr_ser() {
        let code = "RUST".to_string();
        let pk =
            PublicKey::from_account_id("GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D")
                .unwrap();
        let asset = Asset::new_credit(code, pk).unwrap();
        let xdr = asset.xdr_base64().unwrap();
        assert_eq!(
            "AAAAAVJVU1QAAAAAsnuvp7wv0ARs15Z8RFDPXJKdbnrzfn7EC/ddPL0FSq0=",
            xdr
        );
    }

    #[test]
    fn test_asset_alphanum12_xdr_ser() {
        let code = "RUSTRUSTRUST".to_string();
        let pk =
            PublicKey::from_account_id("GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D")
                .unwrap();
        let asset = Asset::new_credit(code, pk).unwrap();
        let xdr = asset.xdr_base64().unwrap();
        assert_eq!(
            "AAAAAlJVU1RSVVNUUlVTVAAAAACye6+nvC/QBGzXlnxEUM9ckp1uevN+fsQL9108vQVKrQ==",
            xdr
        );
    }

    #[test]
    fn test_asset_native_xdr_de() {
        let expected = Asset::new_native();
        let asset = Asset::from_xdr_base64("AAAAAA==").unwrap();
        assert_eq!(expected, asset);
    }

    #[test]
    fn test_asset_alphanum4_xdr_de() {
        let code = "RUST".to_string();
        let pk =
            PublicKey::from_account_id("GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D")
                .unwrap();
        let expected = Asset::new_credit(code, pk).unwrap();
        let asset =
            Asset::from_xdr_base64("AAAAAVJVU1QAAAAAsnuvp7wv0ARs15Z8RFDPXJKdbnrzfn7EC/ddPL0FSq0=")
                .unwrap();
        assert_eq!(expected, asset);
    }

    #[test]
    fn test_asset_alphanum12_xdr_de() {
        let code = "RUSTRUSTRUST".to_string();
        let pk =
            PublicKey::from_account_id("GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D")
                .unwrap();
        let expected = Asset::new_credit(code, pk).unwrap();
        let asset = Asset::from_xdr_base64(
            "AAAAAlJVU1RSVVNUUlVTVAAAAACye6+nvC/QBGzXlnxEUM9ckp1uevN+fsQL9108vQVKrQ==",
        )
        .unwrap();
        assert_eq!(expected, asset);
    }
}
