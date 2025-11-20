# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2025-11-20
### Added
- Add `InvokeHostFunctionOperation` and related types and enum values
- Add `ExtendFootprintTtlOperation` and related types and enum values
- Add `RestoreFootprintOperation` and related types and enum values
- Add `ContractData`, `ContractCode`, `ConfigSetting` and `Ttl` to the `LedgerKey` enum
- Add `Error` variants: `InvalidPayload`, `UnsupportedFeature`
- Add `DalekKeyPair` (feature `dalek`)

### Changed
- BREAKING: Replaced feature `sodium_oxide` with `dalek` since sodiumoxide is no longer maintained
- BREAKING: Replaced generated code with `stellar-xdr` crate
- Update dependency's versions

### Removed
- Remove `SodiumKeyPair` (use `DalekKeyPair`)
- Remove `Error::SodiumInitFailed`

## [0.6.0] - 2023-11-03
### Added
- Add tests for `BeginSponsoringFutureReservesOperation`
- Add `ChangeTrustAsset`
- Add `ClawbackOperation`
- Add `ClawbackClaimableBalanceOperation`
- Add `LiquidityPoolDepositOperation`
- Add `LiquidityPoolWithdrawOperation`
- Add `LiquidityPool` to the `LedgerKey` enum
- Add `SetTrustlineFlagsOperation`
- Add support for revoking liquidity pool sponsorship to `RevokeSponsorshipOperation`
- Add `AUTH_CLAWBACK_ENABLED` to `AccountFlags`
- Add `TRUSTLINE_CLAWBACK_ENABLED` to `TrustLineFlags`
- Add `TrustlineAsset`
- Add `InvalidLiquidityPoolIdLength` to `Error`
- Add `LedgerKey::LiquidityPool`
- Add `LiquidityPoolId`
- Add `LiquidityPoolConstantFeeParameters`
- Add `LiquidityPoolConstantFee`
- Add `LiquidityPool`
- Add `InnerOperationResult::ClaimClaimableBalance`
- Add `InnerOperationResult::Clawback`
- Add `InnerOperationResult::ClawbackClaimableBalance`
- Add `InnerOperationResult::SetTrustLineFlags`
- Add `InnerOperationResult::LiquidityPoolDeposit`
- Add `InnerOperationResult::LiquidityPoolWithdraw`
- Add `InnerTransactionResult::FeeBumpInnerSuccess`
- Add `InnerTransactionResult::FeeBumpInnerFailed`

### Changed
- BREAKING: Use `ChangeTrustAsset` for `ChangeTrust` operation
- BREAKING: Use `TrustLineAsset` instead of `Asset` in `LedgerKey::Trustline`
- BREAKING: Update `xdr_generated.rs` from the current stellar X files.
- BREAKING: Use traits from the `ed25519` crate to allow using any ed25519
  backend (libsodium, dalek, etc or even a custom signing implementation). The
  use of sodiumoxide is now behind the feature `sodium_oxide`, which is enabled
  by default.
- Update dependency's versions

### Fixed
- Fix `ChangeTrustOperationBuilder` to allow explicitly passing limit 0.


## [0.5.0] - 2021-01-15
### Added
 - Add support for CAP-0023.
 - Add `CreateClaimableBalanceOperation`.
 - Add `ClaimClaimableBalanceOperation`.
 - Add `Claimant` and `ClaimPredicate`.
 - Add `BeginSponsoringFutureReservesOperation`.
 - Add `EndSponsoringFutureReservesOperation`.
 - Add `RevokeSponsorshipOperation`.
 - Add `TransactionResult` and `OperationResult`.


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
