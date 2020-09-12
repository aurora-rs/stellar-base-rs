# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
 - Add `CreateClaimableBalanceOperation`.
 - Add `Claimant` and `ClaimPredicate`.


## [0.4.1] - 2020-08-05
### Added
 - `PublicKey` implements `Display` and `FromStr`.


## [0.4.0] - 2020-07-18
### Added
 - Add `sign_hashx` and `decorated_signature_from_preimage` to
   `FeeBumpTransaction` and `TransactionEnvelope`.

### Changed
 - Rename `TransactionBuilder::to_transaction` to
   `TransactionBuilder::into_transaction`.


## [0.3.0] - 2020-07-18
### Added
 - Add `Signer` and `SignerKey` related methods.

### Changed
 - Change `PreAuthTxHash` struct.
 - Change `HashX` struct.


## [0.2.0] - 2020-07-15
### Added
 - Add several mutable accessors to structs fields.

### Changed
 - Rename enum accessors to `as_`.
 - Move builders inside structs.


## [0.1.0] - 2020-07-14
### Added
 - Build and sign transactions.
 - Base Stellar types.
 - Decode and encode account ids, secret seeds, transaction hashes and
   muxed accounts.
 - Add generated XDR types.
