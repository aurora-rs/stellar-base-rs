use xdr_rs_serialize::error::Error as XdrError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error that can occur when parsing a key.
    #[error("invalid str key")]
    InvalidStrKey,
    /// Invalid version byte in key.
    #[error("invalid str key version byte")]
    InvalidStrKeyVersionByte,
    /// Invalid checksum in key.
    #[error("invalid str key checksum")]
    InvalidStrKeyChecksum,
    /// Invalid keypair seed.
    #[error("invalid seed")]
    InvalidSeed,
    /// Invalid Asset code.
    #[error("invalid asset code")]
    InvalidAssetCode,
    /// Invalid signature.
    #[error("invalid signature")]
    InvalidSignature,
    /// Invalid signature hint.
    #[error("invalid signature hint")]
    InvalidSignatureHint,
    /// Invalid memo text: too long.
    #[error("memo text too long")]
    InvalidMemoText,
    /// Invalid memo hash: too long.
    #[error("memo hash too long")]
    InvalidMemoHash,
    /// Invalid memo return hash: too long.
    #[error("memo return hash too long")]
    InvalidMemoReturn,
    /// Error that can occur when parsing amounts from stroops.
    #[error("invalid stroops amount")]
    InvalidStroopsAmount,
    /// Error that can occur when converting stroops to unsigned amount.
    #[error("stroops amount is negative")]
    NegativeStroops,
    /// Error that can occur when converting an amount with more than 7 digits.
    #[error("invalid amount scale")]
    InvalidAmountScale,
    /// Error parsing price.
    #[error("parse price error")]
    ParsePriceError,
    /// Invalid network id: too long.
    #[error("invalid network id")]
    InvalidNetworkId,
    /// Invalid public key.
    #[error("invalid public key")]
    InvalidPublicKey,
    /// Invalid time bounds.
    #[error("invalid time bounds")]
    InvalidTimeBounds,
    /// Error that can occur when parsing amounts.
    #[error("error parsing amount")]
    ParseAmountError(#[from] rust_decimal::Error),
    /// Error that occurs when building operations.
    #[error("error building operation")]
    InvalidOperation(String),
    /// Error that occurs when building a transaction with too many operations.
    #[error("transaction has too many operations")]
    TooManyOperations,
    /// Error that occurs when building a transaction with no operations.
    #[error("transaction has no operations")]
    MissingOperations,
    /// Transaction fee is too low.
    #[error("transaction fee too low")]
    TransactionFeeTooLow,
    /// Home domain is too long.
    #[error("home domain too long")]
    HomeDomainTooLong,
    /// Invalid account flags.
    #[error("invalid account flags")]
    InvalidAccountFlags,
    /// Invalid trust line flags.
    #[error("invalid trust line flags")]
    InvalidTrustLineFlags,
    /// Transaction fee overflow.
    #[error("transaction fee overflow")]
    TransactionFeeOverflow,
    /// Xdr serialization error
    #[error("xdr serialization error")]
    XdrError(XdrError),
    /// Base64 decode error
    #[error("base64 decode error")]
    Base64DecodeError(#[from] base64::DecodeError),
}
