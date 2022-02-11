use std::convert::TryInto;

use crate::crypto::strkey;
use crate::error::{Error, Result};
use crate::xdr;

/// The public key of the account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicKey(pub [u8; 32]);

impl PublicKey {
    /// Create from `account_id`, e.g. `GB3KJPLFUYN5VL6R3GU3EGCGVCKFDSD7BEDX42HWG5BWFKB3KQGJJRMA`.
    pub fn from_account_id(account_id: &str) -> Result<PublicKey> {
        let bytes = strkey::decode_account_id(&account_id)?;
        Self::from_slice(&bytes)
    }

    /// Create from raw bytes.
    pub fn from_slice(data: &[u8]) -> Result<PublicKey> {
        // let key = ed25519::PublicKey::from_slice(&data).ok_or(Error::InvalidPublicKey)?;
        Ok(PublicKey(
            data.try_into().map_err(|_| Error::InvalidPublicKey)?,
        ))
    }

    pub fn account_id(&self) -> String {
        strkey::encode_account_id(&self.0)
    }

    pub fn into_muxed_account(self, id: u64) -> MuxedAccount {
        let inner = MuxedEd25519PublicKey { key: self, id };
        MuxedAccount::MuxedEd25519(inner)
    }

    pub fn to_muxed_account(&self, id: u64) -> MuxedAccount {
        self.clone().into_muxed_account(id)
    }

    pub fn to_xdr_uint256(&self) -> Result<xdr::Uint256> {
        let bytes = self.as_bytes().to_vec();
        Ok(xdr::Uint256::new(bytes))
    }

    pub fn to_xdr_public_key(&self) -> Result<xdr::PublicKey> {
        let uint256 = self.to_xdr_uint256()?;
        Ok(xdr::PublicKey::PublicKeyTypeEd25519(uint256))
    }

    pub fn to_xdr_account_id(&self) -> Result<xdr::AccountId> {
        let public_key = self.to_xdr_public_key()?;
        Ok(xdr::AccountId::new(public_key))
    }

    pub fn from_xdr_public_key(x: &xdr::PublicKey) -> Result<PublicKey> {
        match x {
            xdr::PublicKey::PublicKeyTypeEd25519(inner) => Self::from_slice(&inner.value),
        }
    }

    pub fn from_xdr_account_id(x: &xdr::AccountId) -> Result<PublicKey> {
        Self::from_xdr_public_key(&x.value)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn to_xdr(&self) -> Result<xdr::MuxedAccount> {
        let uint256 = self.to_xdr_uint256()?;
        Ok(xdr::MuxedAccount::KeyTypeEd25519(uint256))
    }
}

/// A public key together with an id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuxedEd25519PublicKey {
    key: PublicKey,
    id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MuxedAccount {
    Ed25519(PublicKey),
    MuxedEd25519(MuxedEd25519PublicKey),
}

impl MuxedEd25519PublicKey {
    pub fn new(key: PublicKey, id: u64) -> MuxedEd25519PublicKey {
        MuxedEd25519PublicKey { key, id }
    }

    pub fn from_account_id(account_id: &str) -> Result<MuxedEd25519PublicKey> {
        let (bytes, id) = strkey::decode_muxed_account(&account_id)?;
        Self::from_slice(&bytes, id)
    }

    /// Create from raw byte and id.
    pub fn from_slice(data: &[u8], id: u64) -> Result<MuxedEd25519PublicKey> {
        let key = PublicKey::from_slice(&data)?;
        Ok(MuxedEd25519PublicKey { key, id })
    }

    /// Return the inner key.
    pub fn public_key(&self) -> &PublicKey {
        &self.key
    }

    pub fn account_id(&self) -> String {
        strkey::encode_muxed_account(&self.key.as_bytes(), self.id)
    }

    pub fn to_xdr(&self) -> Result<xdr::MuxedAccount> {
        let uint64 = xdr::Uint64::new(self.id);
        let uint256 = self.key.to_xdr_uint256()?;
        let muxed = xdr::MuxedAccountMed25519 {
            id: uint64,
            ed25519: uint256,
        };
        Ok(xdr::MuxedAccount::KeyTypeMuxedEd25519(muxed))
    }
}

impl MuxedAccount {
    pub fn account_id(&self) -> String {
        match self {
            MuxedAccount::Ed25519(pk) => pk.account_id(),
            MuxedAccount::MuxedEd25519(mx) => mx.account_id(),
        }
    }

    pub fn to_xdr(&self) -> Result<xdr::MuxedAccount> {
        match self {
            MuxedAccount::Ed25519(pk) => pk.to_xdr(),
            MuxedAccount::MuxedEd25519(mx) => mx.to_xdr(),
        }
    }

    pub fn from_xdr(x: &xdr::MuxedAccount) -> Result<MuxedAccount> {
        match x {
            xdr::MuxedAccount::KeyTypeEd25519(buf) => {
                let inner = PublicKey::from_slice(&buf.value)?;
                Ok(MuxedAccount::Ed25519(inner))
            }
            xdr::MuxedAccount::KeyTypeMuxedEd25519(mx) => {
                let inner = MuxedEd25519PublicKey::from_slice(&mx.ed25519.value, mx.id.value)?;
                Ok(MuxedAccount::MuxedEd25519(inner))
            }
        }
    }
}

impl From<PublicKey> for MuxedAccount {
    fn from(pk: PublicKey) -> Self {
        MuxedAccount::Ed25519(pk)
    }
}

impl From<MuxedEd25519PublicKey> for MuxedAccount {
    fn from(muxed: MuxedEd25519PublicKey) -> Self {
        MuxedAccount::MuxedEd25519(muxed)
    }
}

impl std::str::FromStr for PublicKey {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pk = PublicKey::from_account_id(s)?;
        Ok(pk)
    }
}

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.account_id())
    }
}
