//! Transaction signatures.
use crate::crypto::{hash, PublicKey, SecretKey};
use crate::error::{Error, Result};
use crate::network::Network;
use crate::transaction::TransactionEnvelope;
use crate::xdr::{self, XDRDeserialize, XDRSerialize};
use sodiumoxide::crypto::sign::ed25519;
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

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
pub struct PreAuthTxHash(Vec<u8>);

/// Hash(x)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashX(Vec<u8>);

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
    /// Signs `data` using the `secret` key.
    pub fn sign(secret: &SecretKey, data: &[u8]) -> Signature {
        let sig = ed25519::sign_detached(data, &secret.inner());
        Signature { sig }
    }

    /// Returns a `Signature` from bytes.
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
    pub fn as_bytes(&self) -> &[u8] {
        &self.sig.0
    }

    /// Verifise the signature againt the `data` and the `public` key.
    /// Returns `true` if the signature is valid, `false` otherwise.
    pub fn verify(&self, public: &PublicKey, data: &[u8]) -> bool {
        ed25519::verify_detached(&self.sig, data, &public.inner())
    }

    /// Returns xdr object.
    pub fn to_xdr(&self) -> Result<xdr::Signature> {
        Ok(xdr::Signature::new(self.to_vec()))
    }

    /// Creates from xdr object.
    pub fn from_xdr(x: &xdr::Signature) -> Result<Signature> {
        Signature::from_slice(&x.value)
    }
}

impl SignatureHint {
    /// Creates a `SignatureHint` with the last 4 bytes of the public key `pk`.
    pub fn from_public_key(pk: &PublicKey) -> SignatureHint {
        let mut hint: [u8; 4] = Default::default();
        let buf = pk.as_bytes();
        let len = buf.len();
        hint.copy_from_slice(&buf[len - 4..len]);
        SignatureHint(hint)
    }

    /// Creates a `SignatureHint` from the byte slice.
    pub fn from_slice(buf: &[u8]) -> Result<SignatureHint> {
        let mut hint: [u8; 4] = Default::default();
        if buf.len() != 4 {
            return Err(Error::InvalidSignatureHint);
        }
        hint.copy_from_slice(&buf[0..4]);
        Ok(SignatureHint(hint))
    }

    /// Converts to `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Returns xdr object.
    pub fn to_xdr(&self) -> Result<xdr::SignatureHint> {
        Ok(xdr::SignatureHint::new(self.to_vec()))
    }

    /// Creates from xdr object.
    pub fn from_xdr(x: &xdr::SignatureHint) -> Result<SignatureHint> {
        SignatureHint::from_slice(&x.value)
    }
}

impl DecoratedSignature {
    /// Creates a new `DecoratedSignature` with `hint` and `signature`.
    pub fn new(hint: SignatureHint, signature: Signature) -> DecoratedSignature {
        DecoratedSignature { hint, signature }
    }

    /// Creates a new `DecoratedSignature` from the pre image.
    pub fn new_from_preimage(preimage: &[u8]) -> Result<DecoratedSignature> {
        let hint = SignatureHint::from_slice(&preimage[preimage.len() - 4..])?;
        let signature = Signature::from_slice(&preimage)?;
        Ok(DecoratedSignature::new(hint, signature))
    }

    /// Returns the decorated signature `hint`.
    pub fn hint(&self) -> &SignatureHint {
        &self.hint
    }

    /// Returns a mutable reference to the decorated signature `hint`.
    pub fn hint_mut(&mut self) -> &mut SignatureHint {
        &mut self.hint
    }

    /// Returns the decorated signature `signature`.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Returns a mutable reference to the decorated signature `signature`.
    pub fn signature_mut(&mut self) -> &mut Signature {
        &mut self.signature
    }

    /// Returns xdr object.
    pub fn to_xdr(&self) -> Result<xdr::DecoratedSignature> {
        let hint = self.hint.to_xdr()?;
        let signature = self.signature.to_xdr()?;
        Ok(xdr::DecoratedSignature { hint, signature })
    }

    /// Creates from xdr object.
    pub fn from_xdr(x: &xdr::DecoratedSignature) -> Result<DecoratedSignature> {
        let hint = SignatureHint::from_xdr(&x.hint)?;
        let signature = Signature::from_xdr(&x.signature)?;
        Ok(DecoratedSignature::new(hint, signature))
    }
}

impl SignerKey {
    /// Creates a `SignerKey` with ed25519 key.
    pub fn new_from_public_key(key: PublicKey) -> SignerKey {
        SignerKey::Ed25519(key)
    }

    /// Creates a `SignerKey` with hash(x) key.
    pub fn new_from_hashx(hashx: HashX) -> SignerKey {
        SignerKey::HashX(hashx)
    }

    /// Creates a `SignerKey` with the hash of the preimage as key.
    pub fn new_with_hashx(preimage: &[u8]) -> SignerKey {
        let hashx = HashX::new_from_preimage(preimage);
        SignerKey::new_from_hashx(hashx)
    }

    /// Creates a `SignerKey` from the pre authorized transaction hash.
    pub fn new_from_pre_authorized_transaction(preauthtx: PreAuthTxHash) -> SignerKey {
        SignerKey::PreAuthTx(preauthtx)
    }

    /// Creates a `SignerKey` from the transaction envelope as pre authorized transaction.
    pub fn new_from_transaction_envelope(
        tx: &TransactionEnvelope,
        network: &Network,
    ) -> Result<SignerKey> {
        let preauthtx = PreAuthTxHash::new_from_transaction_envelope(&tx, &network)?;
        Ok(SignerKey::new_from_pre_authorized_transaction(preauthtx))
    }

    /// If the signer is a Ed25519 key, returns its value. Returns None otherwise.
    pub fn as_ed25519(&self) -> Option<&PublicKey> {
        match *self {
            SignerKey::Ed25519(ref key) => Some(key),
            _ => None,
        }
    }

    /// If the signer is a Ed25519 key, returns its mutable value. Returns None otherwise.
    pub fn as_ed25519_mut(&mut self) -> Option<&mut PublicKey> {
        match *self {
            SignerKey::Ed25519(ref mut key) => Some(key),
            _ => None,
        }
    }

    /// Returns true if the signer is a Ed25519 key.
    pub fn is_ed25519(&self) -> bool {
        self.as_ed25519().is_some()
    }

    /// If the signer is a PreAuthTx, returns its value. Returns None otherwise.
    pub fn as_pre_authorized_transaction(&self) -> Option<&PreAuthTxHash> {
        match *self {
            SignerKey::PreAuthTx(ref hash) => Some(hash),
            _ => None,
        }
    }

    /// If the signer is a PreAuthTx, returns its mutable value. Returns None otherwise.
    pub fn as_pre_authorized_transaction_mut(&mut self) -> Option<&mut PreAuthTxHash> {
        match *self {
            SignerKey::PreAuthTx(ref mut hash) => Some(hash),
            _ => None,
        }
    }

    /// Returns true if the signer is a PreAuthTx.
    pub fn is_pre_authorized_transaction(&self) -> bool {
        self.as_pre_authorized_transaction().is_some()
    }

    /// If the signer is a HashX, returns its value. Returns None otherwise.
    pub fn as_hashx(&self) -> Option<&HashX> {
        match *self {
            SignerKey::HashX(ref hash) => Some(hash),
            _ => None,
        }
    }

    /// If the signer is a HashX, returns its mutable value. Returns None otherwise.
    pub fn as_hashx_mut(&mut self) -> Option<&mut HashX> {
        match *self {
            SignerKey::HashX(ref mut hash) => Some(hash),
            _ => None,
        }
    }

    /// Returns true if the signer is a HashX.
    pub fn is_hashx(&self) -> bool {
        self.as_hashx().is_some()
    }

    /// Returns the xdr object.
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

    /// Creates from xdr object.
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
    /// Creates a new `Signer` with `key` and `weight`.
    pub fn new(key: SignerKey, weight: u32) -> Signer {
        Signer { key, weight }
    }

    /// Returns the key.
    pub fn key(&self) -> &SignerKey {
        &self.key
    }

    /// Returns a mutable reference to the key.
    pub fn key_mut(&mut self) -> &mut SignerKey {
        &mut self.key
    }

    /// Returns the weight.
    pub fn weight(&self) -> &u32 {
        &self.weight
    }

    /// Returns a mutable reference to the weight.
    pub fn weight_mut(&mut self) -> &mut u32 {
        &mut self.weight
    }

    /// Returns xdr object.
    pub fn to_xdr(&self) -> Result<xdr::Signer> {
        let key = self.key.to_xdr()?;
        let weight = xdr::Uint32::new(self.weight);
        Ok(xdr::Signer { key, weight })
    }

    /// Creates from xdr object.
    pub fn from_xdr(x: &xdr::Signer) -> Result<Signer> {
        let weight = x.weight.value;
        let key = SignerKey::from_xdr(&x.key)?;
        Ok(Signer { key, weight })
    }
}

impl PreAuthTxHash {
    /// Creates a `PreAuthTxHash` from the transaction hash.
    ///
    /// The `hash` must be exactly 32 bytes.
    pub fn new(hash: Vec<u8>) -> Result<PreAuthTxHash> {
        if hash.len() != 32 {
            return Err(Error::InvalidPreAuthTx);
        }
        Ok(PreAuthTxHash(hash))
    }

    /// Creates a `PreAuthTxHash` from the transaction envelope.
    pub fn new_from_transaction_envelope(
        tx: &TransactionEnvelope,
        network: &Network,
    ) -> Result<PreAuthTxHash> {
        let hash = tx.hash(&network)?;
        PreAuthTxHash::new(hash)
    }

    /// Returns the pre authorized transaction hash as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl HashX {
    /// Creates a `HashX` from a vector of bytes.
    ///
    /// `hashx` must be exactly 32 bytes.
    pub fn new(hashx: Vec<u8>) -> Result<HashX> {
        if hashx.len() != 32 {
            return Err(Error::InvalidHashX);
        }
        Ok(HashX(hashx))
    }

    /// Creates a `HashX` from the `preimage`.
    pub fn new_from_preimage(preimage: &[u8]) -> HashX {
        // hash always returns a 32 byte value.
        // no need to check length.
        HashX(hash(preimage))
    }

    /// Returns the hashx as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl XDRSerialize for SignerKey {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr_signer = self.to_xdr()?;
        xdr_signer.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for SignerKey {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_signer, bytes_read) =
            xdr::SignerKey::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = SignerKey::from_xdr(&xdr_signer)?;
        Ok((res, bytes_read))
    }
}

impl XDRSerialize for Signer {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr_signer = self.to_xdr()?;
        xdr_signer.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for Signer {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_signer, bytes_read) = xdr::Signer::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = Signer::from_xdr(&xdr_signer)?;
        Ok((res, bytes_read))
    }
}

#[cfg(test)]
mod tests {
    use super::SignerKey;
    use crate::crypto::PublicKey;
    use crate::network::Network;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use base64;

    #[test]
    fn test_signer_key_from_public_key() {
        let key =
            PublicKey::from_account_id("GCEE2MAVLB3D5J64TTHR3T4ZYK4BZJEYIPE7FMG4NAXHY3VQRHW55BNX")
                .unwrap();
        let signer_key = SignerKey::new_from_public_key(key);
        assert!(signer_key.is_ed25519());
        assert!(!signer_key.is_pre_authorized_transaction());
        assert!(!signer_key.is_hashx());

        let xdr = signer_key.xdr_base64().unwrap();
        let expected_xdr = "AAAAAIhNMBVYdj6n3JzPHc+ZwrgcpJhDyfKw3GgufG6wie3e";
        assert_eq!(expected_xdr, xdr);

        let back = SignerKey::from_xdr_base64(&xdr).unwrap();
        assert_eq!(back, signer_key);
    }

    #[test]
    fn test_signer_key_with_hashx() {
        let data = "hello".to_string();
        let signer_key = SignerKey::new_with_hashx(data.as_bytes());

        assert!(!signer_key.is_ed25519());
        assert!(!signer_key.is_pre_authorized_transaction());
        assert!(signer_key.is_hashx());

        let hashx = signer_key.as_hashx().unwrap();

        assert_eq!(
            "LPJNul+wow4m6DsqxbninhsWHlwfp0JecwQzYpOLmCQ=".to_string(),
            base64::encode(hashx.as_bytes())
        );

        let xdr = signer_key.xdr_base64().unwrap();
        let expected_xdr = "AAAAAizyTbpfsKMOJug7KsW54p4bFh5cH6dCXnMEM2KTi5gk";
        assert_eq!(expected_xdr, xdr);

        let back = SignerKey::from_xdr_base64(&xdr).unwrap();
        assert_eq!(back, signer_key);
    }

    #[test]
    fn test_signer_key_with_pre_authorized_transaction() {
        let tx = TransactionEnvelope::from_xdr_base64("AAAAAgAAAACITTAVWHY+p9yczx3PmcK4HKSYQ8nysNxoLnxusInt3gAAAGQAAAAAAAAAewAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAABQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
        let signer_key =
            SignerKey::new_from_transaction_envelope(&tx, &Network::new_test()).unwrap();

        assert!(!signer_key.is_ed25519());
        assert!(signer_key.is_pre_authorized_transaction());
        assert!(!signer_key.is_hashx());

        let tx_hash = signer_key
            .as_pre_authorized_transaction()
            .unwrap()
            .as_bytes();
        // Checked in stellar laboraty
        assert_eq!(
            "xkhj28AGwJ4ykWcbjN4347wQFhOKXg1qKFKwiXiKtzY=",
            base64::encode(tx_hash)
        );

        let xdr = signer_key.xdr_base64().unwrap();
        let expected_xdr = "AAAAAcZIY9vABsCeMpFnG4zeN+O8EBYTil4NaihSsIl4irc2";
        assert_eq!(expected_xdr, xdr);

        let back = SignerKey::from_xdr_base64(&xdr).unwrap();
        assert_eq!(back, signer_key);
    }
}
