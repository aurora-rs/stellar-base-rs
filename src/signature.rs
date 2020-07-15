use crate::crypto::{PublicKey, SecretKey};
use crate::error::{Error, Result};
use crate::xdr;
use sodiumoxide::crypto::sign::ed25519;

/// A signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    sig: ed25519::Signature,
}

/// Last 4 bytes of a public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureHint(pub [u8; 4]);

/// A `Signature` together with the last 4 bytes of the public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoratedSignature {
    hint: SignatureHint,
    signature: Signature,
}

/// A pre authorized transaction hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreAuthTxHash(pub Vec<u8>);

/// Hash(x)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashX(pub Vec<u8>);

/// A transaction signer key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignerKey {
    Ed25519(PublicKey),
    PreAuthTx(PreAuthTxHash),
    HashX(HashX),
}

/// A transaction signer key with its weight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signer {
    key: SignerKey,
    weight: u32,
}

impl Signature {
    /// Sign `data` using the `secret` key.
    pub fn sign(secret: &SecretKey, data: &[u8]) -> Signature {
        let sig = ed25519::sign_detached(data, &secret.inner());
        Signature { sig }
    }

    /// Return a `Signature` from bytes.
    pub fn from_slice(sb: &[u8]) -> Result<Signature> {
        let sig = ed25519::Signature::from_slice(sb).ok_or(Error::InvalidSignature)?;
        Ok(Signature { sig })
    }

    /// Length in bytes of the signature.
    pub fn len(&self) -> usize {
        self.sig.0.len()
    }

    /// Returns `true` if the signature has no bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert to `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.sig.0.to_vec()
    }

    /// Inner buffer as slice.
    pub fn buf(&self) -> &[u8] {
        &self.sig.0
    }

    /// Verify the signature againt the `data` and the `public` key.
    /// Return `true` if the signature is valid, `false` otherwise.
    pub fn verify(&self, public: &PublicKey, data: &[u8]) -> bool {
        ed25519::verify_detached(&self.sig, data, &public.inner())
    }

    pub fn to_xdr(&self) -> Result<xdr::Signature> {
        Ok(xdr::Signature::new(self.to_vec()))
    }

    pub fn from_xdr(x: &xdr::Signature) -> Result<Signature> {
        Signature::from_slice(&x.value)
    }
}

impl SignatureHint {
    /// Create a `SignatureHint` with the last 4 bytes of the public key `pk`.
    pub fn from_public_key(pk: &PublicKey) -> SignatureHint {
        let mut hint: [u8; 4] = Default::default();
        let buf = pk.as_bytes();
        let len = buf.len();
        hint.copy_from_slice(&buf[len - 4..len]);
        SignatureHint(hint)
    }

    pub fn from_slice(buf: &[u8]) -> Result<SignatureHint> {
        let mut hint: [u8; 4] = Default::default();
        if buf.len() != 4 {
            return Err(Error::InvalidSignatureHint);
        }
        hint.copy_from_slice(&buf[0..4]);
        Ok(SignatureHint(hint))
    }

    /// Convert to `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_xdr(&self) -> Result<xdr::SignatureHint> {
        Ok(xdr::SignatureHint::new(self.to_vec()))
    }

    pub fn from_xdr(x: &xdr::SignatureHint) -> Result<SignatureHint> {
        SignatureHint::from_slice(&x.value)
    }
}

impl DecoratedSignature {
    /// Create a new `DecoratedSignature` with `hint` and `signature`.
    pub fn new(hint: SignatureHint, signature: Signature) -> DecoratedSignature {
        DecoratedSignature { hint, signature }
    }

    /// Return the decorated signature `hint`.
    pub fn hint(&self) -> &SignatureHint {
        &self.hint
    }

    /// Return the decorated signature `signature`.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub fn to_xdr(&self) -> Result<xdr::DecoratedSignature> {
        let hint = self.hint.to_xdr()?;
        let signature = self.signature.to_xdr()?;
        Ok(xdr::DecoratedSignature { hint, signature })
    }

    pub fn from_xdr(x: &xdr::DecoratedSignature) -> Result<DecoratedSignature> {
        let hint = SignatureHint::from_xdr(&x.hint)?;
        let signature = Signature::from_xdr(&x.signature)?;
        Ok(DecoratedSignature::new(hint, signature))
    }
}

impl SignerKey {
    pub fn to_xdr(&self) -> Result<xdr::SignerKey> {
        match self {
            SignerKey::Ed25519(pk) => {
                let key_bytes = pk.as_bytes();
                let inner = xdr::Uint256::new(key_bytes.to_vec());
                Ok(xdr::SignerKey::SignerKeyTypeEd25519(inner))
            }
            SignerKey::PreAuthTx(hash) => {
                let inner = xdr::Uint256::new(hash.0.to_vec());
                Ok(xdr::SignerKey::SignerKeyTypePreAuthTx(inner))
            }
            SignerKey::HashX(hash) => {
                let inner = xdr::Uint256::new(hash.0.to_vec());
                Ok(xdr::SignerKey::SignerKeyTypeHashX(inner))
            }
        }
    }

    pub fn from_xdr(x: &xdr::SignerKey) -> Result<SignerKey> {
        match x {
            xdr::SignerKey::SignerKeyTypeEd25519(bytes) => {
                let pk = PublicKey::from_slice(&bytes.value)?;
                Ok(SignerKey::Ed25519(pk))
            }
            xdr::SignerKey::SignerKeyTypePreAuthTx(bytes) => {
                let inner = PreAuthTxHash(bytes.value.to_vec());
                Ok(SignerKey::PreAuthTx(inner))
            }
            xdr::SignerKey::SignerKeyTypeHashX(bytes) => {
                let inner = HashX(bytes.value.to_vec());
                Ok(SignerKey::HashX(inner))
            }
        }
    }
}

impl Signer {
    pub fn new(key: SignerKey, weight: u32) -> Signer {
        Signer { key, weight }
    }

    pub fn to_xdr(&self) -> Result<xdr::Signer> {
        let key = self.key.to_xdr()?;
        let weight = xdr::Uint32::new(self.weight);
        Ok(xdr::Signer { key, weight })
    }

    pub fn from_xdr(x: &xdr::Signer) -> Result<Signer> {
        let weight = x.weight.value;
        let key = SignerKey::from_xdr(&x.key)?;
        Ok(Signer { key, weight })
    }
}
