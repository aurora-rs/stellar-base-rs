//! Cryptographic functions.

mod public_key;
mod signature;
#[cfg(feature = "sodium_oxide")]
mod sodium_oxide;
mod strkey;

use crate::error::Error;
use sha2::Digest;
use std::convert::TryInto;

pub use self::public_key::{MuxedAccount, MuxedEd25519PublicKey, PublicKey};
pub use self::signature::*;
pub use self::strkey::*;
pub use ed25519::signature::{Signer as Ed25519Signer, Verifier as Ed25519Verifier};
pub use ed25519::Signature;
#[cfg(feature = "sodium_oxide")]
pub use sodium_oxide::*;

/// Compute sha256 hash of `m`.
pub fn hash(m: &[u8]) -> Vec<u8> {
    sha2::Sha256::digest(m).to_vec()
}

#[derive(Debug, Clone, PartialEq)]
pub struct SigningKey<S>
where
    S: Ed25519Signer<Signature>,
{
    pub signing_key: S,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VerifyKey<V>
where
    V: Ed25519Verifier<Signature> + AsRef<[u8]>,
{
    pub verify_key: V,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyPair<S, V>
where
    S: Ed25519Signer<Signature>,
    V: Ed25519Verifier<Signature> + AsRef<[u8]>,
{
    pub signer: SigningKey<S>,
    pub verifier: VerifyKey<V>,
}

impl<S, V> KeyPair<S, V>
where
    S: Ed25519Signer<Signature>,
    V: Ed25519Verifier<Signature> + AsRef<[u8]>,
{
    pub fn new(signer: S, verifier: V) -> KeyPair<S, V> {
        KeyPair {
            signer: SigningKey {
                signing_key: signer,
            },
            verifier: VerifyKey {
                verify_key: verifier,
            },
        }
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.signer.signing_key.sign(msg)
    }

    pub fn verify(&self, msg: &[u8], sig: &Signature) -> Result<(), ed25519::Error> {
        self.verifier.verify_key.verify(msg, sig)
    }

    pub fn public_key(&self) -> Result<PublicKey, Error> {
        Ok(PublicKey(
            self.verifier
                .verify_key
                .as_ref()
                .try_into()
                .map_err(|_| Error::InvalidPublicKey)?,
        ))
    }

    /// Sign the `message` together with the signature hint.
    pub fn sign_decorated(&self, message: &[u8]) -> DecoratedSignature {
        let hint = self.signature_hint();
        let signature = self.signer.signing_key.sign(message);
        DecoratedSignature::new(hint, signature)
    }

    /// Return the signature hint, that is the last 4 bytes of the public key.
    pub fn signature_hint(&self) -> SignatureHint {
        SignatureHint::from_public_key(self.verifier.verify_key.as_ref())
    }
}
