//! Cryptographic functions using ed25519-dalek.
//!
//! ```rust
//! use stellar_base::crypto;
//! // Use DalekKeyPair for key generation and signing
//! ```

mod keypair;

pub use self::keypair::*;
