//! # Stellar Base
//!
//! The `stellar-base` crate provides low level Stellar types.
//!
//!  - Create and sign transactions
//!  - Encode and decode Stellar objects from XDR
//!
//! If you are looking for a way to interact with Horizon, take a
//! look at [`stellar-horizon`](https://docs.rs/stellar-horizon).
//!
//!
//! ## Creating a KeyPair
//!
//! A Stellar KeyPair contains a Secretkey and a PublicKey. You can
//! use the PublicKey to identify the Stellar account, and the SecretKey
//! to sign transactions.
//!
//! ```rust
//! use stellar_base::crypto::KeyPair;
//!
//! # fn run() -> stellar_base::error::Result<()> {
//! let random_kp = KeyPair::random()?;
//! println!("Account Id = {}", random_kp.public_key().account_id());
//! # Ok(())
//! # }
//! ```
//!
//!
//! # Creating and signing a Transaction
//!
//! You can use this crate to create Stellar transactions and serialize them
//! to XDR.
//!
//! ```rust
//! use stellar_base::amount::Amount;
//! use stellar_base::asset::Asset;
//! use stellar_base::crypto::{KeyPair, PublicKey};
//! use stellar_base::memo::Memo;
//! use stellar_base::network::Network;
//! use stellar_base::operations::Operation;
//! use stellar_base::transaction::{Transaction, MIN_BASE_FEE};
//! use stellar_base::xdr::XDRSerialize;
//! use std::str::FromStr;
//!
//! # fn run() -> stellar_base::error::Result<()> {
//! let source_kp = KeyPair::random()?;
//! let destination = PublicKey::from_account_id("GATTMQEODSDX45WZK2JFIYETXWYCU5GRJ5I3Z7P2UDYD6YFVONDM4CX4")?;
//!
//! let payment_amount = Amount::from_str("13.12")?;
//!
//! let payment = Operation::new_payment()
//!     .with_destination(destination.clone())
//!     .with_amount(payment_amount)?
//!     .with_asset(Asset::new_native())
//!     .build()?;
//!
//! let mut tx = Transaction::builder(source_kp.public_key().clone(), 1234, MIN_BASE_FEE)
//!     .with_memo(Memo::new_id(7483792))
//!     .add_operation(payment)
//!     .into_transaction()?;
//!
//! tx.sign(&source_kp, &Network::new_test());
//! let xdr = tx.into_envelope().xdr_base64()?;
//! println!("Xdr = {}", xdr);
//! # Ok(())
//! # }
//! ```
//!
#[macro_use]
extern crate xdr_rs_serialize_derive;
#[macro_use]
extern crate bitflags;
extern crate chrono;
extern crate json;
extern crate xdr_rs_serialize;

pub mod account;
pub mod amount;
pub mod asset;
pub mod claim;
pub mod crypto;
pub mod error;
pub mod ledger;
pub mod liquidity_pool;
pub mod memo;
pub mod network;
pub mod operation_result;
pub mod operations;
pub mod signature;
pub mod time_bounds;
pub mod transaction;
pub mod transaction_result;
pub mod xdr;
mod xdr_generated;

pub use self::asset::Asset;
pub use self::crypto::{KeyPair, PublicKey};
pub use self::memo::Memo;
pub use self::network::Network;
pub use self::operation_result::OperationResult;
pub use self::operations::Operation;
pub use self::transaction::Transaction;
pub use self::transaction_result::TransactionResult;
