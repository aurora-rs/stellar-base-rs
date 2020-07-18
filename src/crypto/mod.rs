//! Cryptographic functions.
//!
//! For long running programs, you should initialize the sodium
//! library for optimal performance.
//!
//! ```rust
//! use stellar_base::crypto;
//!
//! # fn run() -> stellar_base::error::Result<()> {
//! crypto::init()?;
//! // your program here.
//! # Ok(())
//! # }
//! ```
use crate::error::{Error, Result};
use sodiumoxide::crypto::hash::sha256;
use sodiumoxide::randombytes;

mod keypair;
mod strkey;

pub use self::keypair::{KeyPair, MuxedAccount, MuxedEd25519PublicKey, PublicKey, SecretKey};
pub use self::strkey::*;

/// Compute sha256 hash of `m`.
pub fn hash(m: &[u8]) -> Vec<u8> {
    let digest = sha256::hash(&m);
    digest.0.to_vec()
}

/// Generate `size` random bytes.
pub fn random_bytes(size: usize) -> Vec<u8> {
    randombytes::randombytes(size)
}

/// Initialize the sodium library and chooses faster version of the primitives
/// if possible.
///
/// `init` also makes `KeyPair::random()` thread-safe.
pub fn init() -> Result<()> {
    sodiumoxide::init().map_err(|_| Error::SodiumInitFailed)
}
