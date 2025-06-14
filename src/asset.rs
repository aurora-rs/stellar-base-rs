//! Assets on the network.
use std::io::{Read, Write};

use crate::crypto::PublicKey;
use crate::error::{Error, Result};
use crate::liquidity_pool::LiquidityPoolId;
use crate::xdr;

/// Represent an asset, either the native asset (XLM) or an asset
/// issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asset {
    /// The native asset (XLM).
    Native,
    /// A non-native asset, identified by asset code/issuer id.
    Credit(CreditAsset),
}

/// Represent an asset associated with a trustline, either a regular asset or a liquidity pool's
/// shares
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustLineAsset {
    Native,
    Credit(CreditAsset),
    PoolShare(LiquidityPoolId),
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
        matches!(*self, Asset::Native)
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
            Asset::Native => Ok(xdr::Asset::Native),
            Asset::Credit(credit) => match credit {
                CreditAsset::AlphaNum4 { code, issuer } => {
                    let code_len = code.len();
                    let mut code_bytes = [0u8; 4];
                    code_bytes[..code_len].copy_from_slice(code.as_bytes());
                    let asset_code = xdr::AssetCode4(code_bytes);
                    let issuer = issuer.to_xdr_account_id()?;
                    let asset_alphanum4 = xdr::AlphaNum4 { asset_code, issuer };
                    Ok(xdr::Asset::CreditAlphanum4(asset_alphanum4))
                }
                CreditAsset::AlphaNum12 { code, issuer } => {
                    let code_len = code.len();
                    let mut code_bytes = [0u8; 12];
                    code_bytes[..code_len].copy_from_slice(code.as_bytes());
                    let asset_code = xdr::AssetCode12(code_bytes);
                    let issuer = issuer.to_xdr_account_id()?;
                    let asset_alphanum12 = xdr::AlphaNum12 { asset_code, issuer };
                    Ok(xdr::Asset::CreditAlphanum12(asset_alphanum12))
                }
            },
        }
    }

    /// Creates an asset from the xdr object.
    pub fn from_xdr(x: &xdr::Asset) -> Result<Asset> {
        match x {
            xdr::Asset::Native => Ok(Asset::new_native()),
            xdr::Asset::CreditAlphanum4(credit) => {
                let issuer = PublicKey::from_xdr_account_id(&credit.issuer)?;
                let code = xdr_code_to_string(&credit.asset_code.0);
                Asset::new_credit(code, issuer)
            }
            xdr::Asset::CreditAlphanum12(credit) => {
                let issuer = PublicKey::from_xdr_account_id(&credit.issuer)?;
                let code = xdr_code_to_string(&credit.asset_code.0);
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
        if (1..=4).contains(&code_len) {
            Ok(CreditAsset::AlphaNum4 { code, issuer })
        } else if (5..=12).contains(&code_len) {
            Ok(CreditAsset::AlphaNum12 { code, issuer })
        } else {
            Err(Error::InvalidAssetCode)
        }
    }

    /// Returns the asset code.
    pub fn code(&self) -> &str {
        match self {
            CreditAsset::AlphaNum4 { code, issuer: _ } => code,
            CreditAsset::AlphaNum12 { code, issuer: _ } => code,
        }
    }

    /// Returns the asset issuer.
    pub fn issuer(&self) -> &PublicKey {
        match self {
            CreditAsset::AlphaNum4 { code: _, issuer } => issuer,
            CreditAsset::AlphaNum12 { code: _, issuer } => issuer,
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

impl TrustLineAsset {
    /// Create the native asset: Lumens.
    pub fn new_native() -> Self {
        Self::Native
    }

    /// Create the asset with `code` issued by `issuer`.
    pub fn new_credit<S>(code: S, issuer: PublicKey) -> Result<Self>
    where
        S: Into<String>,
    {
        let code = code.into();
        let inner = CreditAsset::new(code, issuer)?;
        Ok(Self::Credit(inner))
    }

    /// Create a trustline asset for a liquidity pool shares.
    pub fn new_pool_share(liquidity_pool_id: LiquidityPoolId) -> Result<Self> {
        Ok(Self::PoolShare(liquidity_pool_id))
    }

    /// Returns true if the asset is a Native. Returns false otherwise.
    pub fn is_native(&self) -> bool {
        matches!(*self, Self::Native)
    }

    /// Returns true if the asset is a Credit. Returns false otherwise.
    pub fn is_credit(&self) -> bool {
        self.as_credit().is_some()
    }

    /// If the asset is a Credit, returns its value. Returns None otherwise
    pub fn as_credit(&self) -> Option<&CreditAsset> {
        match *self {
            Self::Credit(ref credit) => Some(credit),
            _ => None,
        }
    }

    /// If the asset is a Credit, returns its mutable value. Returns None otherwise
    pub fn as_credit_mut(&mut self) -> Option<&mut CreditAsset> {
        match *self {
            Self::Credit(ref mut credit) => Some(credit),
            _ => None,
        }
    }

    pub fn is_pool_share(&self) -> bool {
        self.as_pool_share().is_some()
    }

    pub fn as_pool_share(&self) -> Option<&LiquidityPoolId> {
        match *self {
            Self::PoolShare(ref pool_id) => Some(pool_id),
            _ => None,
        }
    }

    pub fn as_pool_share_mut(&mut self) -> Option<&mut LiquidityPoolId> {
        match *self {
            Self::PoolShare(ref mut pool_id) => Some(pool_id),
            _ => None,
        }
    }

    /// Returns the trustline asset xdr object.
    pub fn to_xdr(&self) -> Result<xdr::TrustLineAsset> {
        match self {
            Self::Native => Ok(xdr::TrustLineAsset::Native),
            Self::Credit(credit) => match credit {
                CreditAsset::AlphaNum4 { code, issuer } => {
                    let code_len = code.len();
                    let mut code_bytes = [0u8; 4];
                    code_bytes[..code_len].copy_from_slice(code.as_bytes());
                    let asset_code = xdr::AssetCode4(code_bytes);
                    let issuer = issuer.to_xdr_account_id()?;
                    let asset_alphanum4 = xdr::AlphaNum4 { asset_code, issuer };
                    Ok(xdr::TrustLineAsset::CreditAlphanum4(asset_alphanum4))
                }
                CreditAsset::AlphaNum12 { code, issuer } => {
                    let code_len = code.len();
                    let mut code_bytes = [0u8; 12];
                    code_bytes[..code_len].copy_from_slice(code.as_bytes());
                    let asset_code = xdr::AssetCode12(code_bytes);
                    let issuer = issuer.to_xdr_account_id()?;
                    let asset_alphanum12 = xdr::AlphaNum12 { asset_code, issuer };
                    Ok(xdr::TrustLineAsset::CreditAlphanum12(asset_alphanum12))
                }
            },
            Self::PoolShare(pool_id) => Ok(xdr::TrustLineAsset::PoolShare(pool_id.to_xdr())),
        }
    }

    /// Creates an asset from the xdr object.
    pub fn from_xdr(x: &xdr::TrustLineAsset) -> Result<Self> {
        match x {
            xdr::TrustLineAsset::Native => Ok(Self::new_native()),
            xdr::TrustLineAsset::CreditAlphanum4(credit) => {
                let issuer = PublicKey::from_xdr_account_id(&credit.issuer)?;
                let code = xdr_code_to_string(&credit.asset_code.0);
                Ok(Self::new_credit(code, issuer)?)
            }
            xdr::TrustLineAsset::CreditAlphanum12(credit) => {
                let issuer = PublicKey::from_xdr_account_id(&credit.issuer)?;
                let code = xdr_code_to_string(&credit.asset_code.0);
                Ok(Self::new_credit(code, issuer)?)
            }
            xdr::TrustLineAsset::PoolShare(pool_id) => {
                let liquidity_pool_id = LiquidityPoolId::from_xdr(pool_id)?;
                Ok(Self::new_pool_share(liquidity_pool_id)?)
            }
        }
    }
}

impl From<Asset> for TrustLineAsset {
    fn from(asset: Asset) -> Self {
        match asset {
            Asset::Native => TrustLineAsset::new_native(),
            Asset::Credit(credit) => {
                TrustLineAsset::new_credit(credit.code(), *credit.issuer()).unwrap()
            }
        }
    }
}

impl xdr::WriteXdr for TrustLineAsset {
    fn write_xdr<W: Write>(&self, w: &mut xdr::Limited<W>) -> xdr::Result<()> {
        let xdr_asset = self.to_xdr().map_err(|_| xdr::Error::Invalid)?;
        xdr_asset.write_xdr(w)
    }
}

impl xdr::ReadXdr for TrustLineAsset {
    fn read_xdr<R: Read>(r: &mut xdr::Limited<R>) -> xdr::Result<Self> {
        let xdr_result = xdr::TrustLineAsset::read_xdr(r)?;
        Self::from_xdr(&xdr_result).map_err(|_| xdr::Error::Invalid)
    }
}

impl xdr::WriteXdr for Asset {
    fn write_xdr<W: Write>(&self, w: &mut xdr::Limited<W>) -> xdr::Result<()> {
        let xdr_asset = self.to_xdr().map_err(|_| xdr::Error::Invalid)?;
        xdr_asset.write_xdr(w)
    }
}

impl xdr::ReadXdr for Asset {
    fn read_xdr<R: Read>(r: &mut xdr::Limited<R>) -> xdr::Result<Self> {
        let xdr_result = xdr::Asset::read_xdr(r)?;
        Self::from_xdr(&xdr_result).map_err(|_| xdr::Error::Invalid)
    }
}

/// Create new String from asset code. Make sure not to copy zero bytes.
pub(crate) fn xdr_code_to_string(x: &[u8]) -> String {
    let mut pos = 0;
    for b in x {
        if *b == 0 {
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
