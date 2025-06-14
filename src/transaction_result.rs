use std::io::Read;

use crate::amount::Stroops;
use crate::error::Result;
use crate::operation_result::OperationResult;
use crate::xdr;

/// Result of a transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionResult {
    FeeBumpSuccess(TransactionResultFeeBumpSuccess),
    FeeBumpFailed(TransactionResultFeeBumpFailed),
    Success(TransactionResultSuccess),
    Failed(TransactionResultFailed),
    TooEarly(TransactionResultTooEarly),
    TooLate(TransactionResultTooLate),
    MissingOperation(TransactionResultMissingOperation),
    BadSequence(TransactionResultBadSequence),
    BadAuth(TransactionResultBadAuth),
    InsufficientBalance(TransactionResultInsufficientBalance),
    NoAccount(TransactionResultNoAccount),
    InsufficientFee(TransactionResultInsufficientFee),
    BadAuthExtra(TransactionResultBadAuthExtra),
    InternalError(TransactionResultInternalError),
    NotSupported(TransactionResultNotSupported),
    BadSponsorship(TransactionResultBadSponsorship),
    BadMinSeqAgeOrGap(TransactionResultBadMinSeqAgeOrGap),
    Malformed(TransactionResultMalformed),
    SorobanInvalid(TransactionResultSorobanInvalid),
}

/// Result of the inner transaction in a FeeBumpTransaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InnerTransactionResult {
    Success(TransactionResultSuccess),
    Failed(TransactionResultFailed),
    TooEarly(TransactionResultTooEarly),
    TooLate(TransactionResultTooLate),
    MissingOperation(TransactionResultMissingOperation),
    BadSequence(TransactionResultBadSequence),
    BadAuth(TransactionResultBadAuth),
    InsufficientBalance(TransactionResultInsufficientBalance),
    NoAccount(TransactionResultNoAccount),
    InsufficientFee(TransactionResultInsufficientFee),
    BadAuthExtra(TransactionResultBadAuthExtra),
    InternalError(TransactionResultInternalError),
    NotSupported(TransactionResultNotSupported),
    BadSponsorship(TransactionResultBadSponsorship),
    BadMinSeqAgeOrGap(TransactionResultBadMinSeqAgeOrGap),
    Malformed(TransactionResultMalformed),
    SorobanInvalid(TransactionResultSorobanInvalid),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultFeeBumpSuccess {
    pub fee_charged: Stroops,
    pub transaction_hash: Vec<u8>,
    pub result: InnerTransactionResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultFeeBumpInnerSuccess {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultFeeBumpFailed {
    pub fee_charged: Stroops,
    pub transaction_hash: Vec<u8>,
    pub result: InnerTransactionResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultFeeBumpInnerFailed {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultSuccess {
    pub fee_charged: Stroops,
    pub results: Vec<OperationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultFailed {
    pub fee_charged: Stroops,
    pub results: Vec<OperationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultTooEarly {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultTooLate {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultMissingOperation {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultBadSequence {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultBadAuth {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultInsufficientBalance {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultNoAccount {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultInsufficientFee {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultBadAuthExtra {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultInternalError {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultNotSupported {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultBadSponsorship {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultBadMinSeqAgeOrGap {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultMalformed {
    pub fee_charged: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResultSorobanInvalid {
    pub fee_charged: Stroops,
}

impl TransactionResult {
    /// If the result is a FeeBumpSuccess, returns its value. Returns None otherwise
    pub fn as_fee_bump_success(&self) -> Option<&TransactionResultFeeBumpSuccess> {
        match *self {
            TransactionResult::FeeBumpSuccess(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a FeeBumpSuccess.
    pub fn is_fee_bump_success(&self) -> bool {
        self.as_fee_bump_success().is_some()
    }

    /// If the result is a FeeBumpFailed, returns its value. Returns None otherwise
    pub fn as_fee_bump_failed(&self) -> Option<&TransactionResultFeeBumpFailed> {
        match *self {
            TransactionResult::FeeBumpFailed(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a FeeBumpFailed.
    pub fn is_fee_bump_failed(&self) -> bool {
        self.as_fee_bump_failed().is_some()
    }

    /// If the result is a Success, returns its value. Returns None otherwise
    pub fn as_success(&self) -> Option<&TransactionResultSuccess> {
        match *self {
            TransactionResult::Success(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a Success.
    pub fn is_success(&self) -> bool {
        self.as_success().is_some()
    }

    /// If the result is a Failed, returns its value. Returns None otherwise
    pub fn as_failed(&self) -> Option<&TransactionResultFailed> {
        match *self {
            TransactionResult::Failed(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a Failed.
    pub fn is_failed(&self) -> bool {
        self.as_failed().is_some()
    }

    /// If the result is a TooEarly, returns its value. Returns None otherwise
    pub fn as_too_early(&self) -> Option<&TransactionResultTooEarly> {
        match *self {
            TransactionResult::TooEarly(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a TooEarly.
    pub fn is_too_early(&self) -> bool {
        self.as_too_early().is_some()
    }

    /// If the result is a TooLate, returns its value. Returns None otherwise
    pub fn as_too_late(&self) -> Option<&TransactionResultTooLate> {
        match *self {
            TransactionResult::TooLate(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a TooLate.
    pub fn is_too_late(&self) -> bool {
        self.as_too_late().is_some()
    }

    /// If the result is a MissingOperation, returns its value. Returns None otherwise
    pub fn as_missing_operation(&self) -> Option<&TransactionResultMissingOperation> {
        match *self {
            TransactionResult::MissingOperation(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a MissingOperation.
    pub fn is_missing_operation(&self) -> bool {
        self.as_missing_operation().is_some()
    }

    /// If the result is a BadSequence, returns its value. Returns None otherwise
    pub fn as_bad_sequence(&self) -> Option<&TransactionResultBadSequence> {
        match *self {
            TransactionResult::BadSequence(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a BadSequence.
    pub fn is_bad_sequence(&self) -> bool {
        self.as_bad_sequence().is_some()
    }

    /// If the result is a BadAuth, returns its value. Returns None otherwise
    pub fn as_bad_auth(&self) -> Option<&TransactionResultBadAuth> {
        match *self {
            TransactionResult::BadAuth(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a BadAuth.
    pub fn is_bad_auth(&self) -> bool {
        self.as_bad_auth().is_some()
    }

    /// If the result is a InsufficientBalance, returns its value. Returns None otherwise
    pub fn as_insufficient_balance(&self) -> Option<&TransactionResultInsufficientBalance> {
        match *self {
            TransactionResult::InsufficientBalance(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a InsufficientBalance.
    pub fn is_insufficient_balance(&self) -> bool {
        self.as_insufficient_balance().is_some()
    }

    /// If the result is a NoAccount, returns its value. Returns None otherwise
    pub fn as_no_account(&self) -> Option<&TransactionResultNoAccount> {
        match *self {
            TransactionResult::NoAccount(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a NoAccount.
    pub fn is_no_account(&self) -> bool {
        self.as_no_account().is_some()
    }

    /// If the result is a InsufficientFee, returns its value. Returns None otherwise
    pub fn as_insufficient_fee(&self) -> Option<&TransactionResultInsufficientFee> {
        match *self {
            TransactionResult::InsufficientFee(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a InsufficientFee.
    pub fn is_insufficient_fee(&self) -> bool {
        self.as_insufficient_fee().is_some()
    }

    /// If the result is a BadAuthExtra, returns its value. Returns None otherwise
    pub fn as_bad_auth_extra(&self) -> Option<&TransactionResultBadAuthExtra> {
        match *self {
            TransactionResult::BadAuthExtra(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a BadAuthExtra.
    pub fn is_bad_auth_extra(&self) -> bool {
        self.as_bad_auth_extra().is_some()
    }

    /// If the result is a InternalError, returns its value. Returns None otherwise
    pub fn as_internal_error(&self) -> Option<&TransactionResultInternalError> {
        match *self {
            TransactionResult::InternalError(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a InternalError.
    pub fn is_internal_error(&self) -> bool {
        self.as_internal_error().is_some()
    }

    /// If the result is a NotSupported, returns its value. Returns None otherwise
    pub fn as_not_supported(&self) -> Option<&TransactionResultNotSupported> {
        match *self {
            TransactionResult::NotSupported(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a NotSupported.
    pub fn is_not_supported(&self) -> bool {
        self.as_not_supported().is_some()
    }

    /// If the result is a BadSponsorship, returns its value. Returns None otherwise
    pub fn as_bad_sponsorship(&self) -> Option<&TransactionResultBadSponsorship> {
        match *self {
            TransactionResult::BadSponsorship(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a BadSponsorship.
    pub fn is_bad_sponsorship(&self) -> bool {
        self.as_bad_sponsorship().is_some()
    }

    /// Creates `TransactionResult` from xdr object.
    pub fn from_xdr(x: &xdr::TransactionResult) -> Result<TransactionResult> {
        let fee_charged = Stroops::from_xdr_int64(x.fee_charged)?;
        match x.result {
            xdr::TransactionResultResult::TxFeeBumpInnerSuccess(ref xdr_inner) => {
                let transaction_hash = xdr_inner.transaction_hash.0.to_vec();
                let result = InnerTransactionResult::from_xdr(&xdr_inner.result)?;
                let inner = TransactionResultFeeBumpSuccess {
                    fee_charged,
                    transaction_hash,
                    result,
                };
                Ok(TransactionResult::FeeBumpSuccess(inner))
            }
            xdr::TransactionResultResult::TxFeeBumpInnerFailed(ref xdr_inner) => {
                let transaction_hash = xdr_inner.transaction_hash.0.to_vec();
                let result = InnerTransactionResult::from_xdr(&xdr_inner.result)?;
                let inner = TransactionResultFeeBumpFailed {
                    fee_charged,
                    transaction_hash,
                    result,
                };
                Ok(TransactionResult::FeeBumpFailed(inner))
            }
            xdr::TransactionResultResult::TxSuccess(ref xdr_results) => {
                let mut results = Vec::new();
                for xdr_result in xdr_results.as_vec() {
                    let result = OperationResult::from_xdr(xdr_result)?;
                    results.push(result);
                }
                let inner = TransactionResultSuccess {
                    fee_charged,
                    results,
                };
                Ok(TransactionResult::Success(inner))
            }
            xdr::TransactionResultResult::TxFailed(ref xdr_results) => {
                let mut results = Vec::new();
                for xdr_result in xdr_results.as_vec() {
                    let result = OperationResult::from_xdr(xdr_result)?;
                    results.push(result);
                }
                let inner = TransactionResultFailed {
                    fee_charged,
                    results,
                };
                Ok(TransactionResult::Failed(inner))
            }
            xdr::TransactionResultResult::TxTooEarly => {
                let inner = TransactionResultTooEarly { fee_charged };
                Ok(TransactionResult::TooEarly(inner))
            }
            xdr::TransactionResultResult::TxTooLate => {
                let inner = TransactionResultTooLate { fee_charged };
                Ok(TransactionResult::TooLate(inner))
            }
            xdr::TransactionResultResult::TxMissingOperation => {
                let inner = TransactionResultMissingOperation { fee_charged };
                Ok(TransactionResult::MissingOperation(inner))
            }
            xdr::TransactionResultResult::TxBadSeq => {
                let inner = TransactionResultBadSequence { fee_charged };
                Ok(TransactionResult::BadSequence(inner))
            }
            xdr::TransactionResultResult::TxBadAuth => {
                let inner = TransactionResultBadAuth { fee_charged };
                Ok(TransactionResult::BadAuth(inner))
            }
            xdr::TransactionResultResult::TxInsufficientBalance => {
                let inner = TransactionResultInsufficientBalance { fee_charged };
                Ok(TransactionResult::InsufficientBalance(inner))
            }
            xdr::TransactionResultResult::TxNoAccount => {
                let inner = TransactionResultNoAccount { fee_charged };
                Ok(TransactionResult::NoAccount(inner))
            }
            xdr::TransactionResultResult::TxInsufficientFee => {
                let inner = TransactionResultInsufficientFee { fee_charged };
                Ok(TransactionResult::InsufficientFee(inner))
            }
            xdr::TransactionResultResult::TxBadAuthExtra => {
                let inner = TransactionResultBadAuthExtra { fee_charged };
                Ok(TransactionResult::BadAuthExtra(inner))
            }
            xdr::TransactionResultResult::TxInternalError => {
                let inner = TransactionResultInternalError { fee_charged };
                Ok(TransactionResult::InternalError(inner))
            }
            xdr::TransactionResultResult::TxNotSupported => {
                let inner = TransactionResultNotSupported { fee_charged };
                Ok(TransactionResult::NotSupported(inner))
            }
            xdr::TransactionResultResult::TxBadSponsorship => {
                let inner = TransactionResultBadSponsorship { fee_charged };
                Ok(TransactionResult::BadSponsorship(inner))
            }
            xdr::TransactionResultResult::TxBadMinSeqAgeOrGap => {
                let inner = TransactionResultBadMinSeqAgeOrGap { fee_charged };
                Ok(TransactionResult::BadMinSeqAgeOrGap(inner))
            }
            xdr::TransactionResultResult::TxMalformed => {
                let inner = TransactionResultMalformed { fee_charged };
                Ok(TransactionResult::Malformed(inner))
            }
            xdr::TransactionResultResult::TxSorobanInvalid => {
                let inner = TransactionResultSorobanInvalid { fee_charged };
                Ok(TransactionResult::SorobanInvalid(inner))
            }
        }
    }
}

impl xdr::ReadXdr for TransactionResult {
    fn read_xdr<R: Read>(r: &mut xdr::Limited<R>) -> xdr::Result<Self> {
        let xdr_result = xdr::TransactionResult::read_xdr(r)?;
        Self::from_xdr(&xdr_result).map_err(|_| xdr::Error::Invalid)
    }
}

impl InnerTransactionResult {
    /// If the result is a Success, returns its value. Returns None otherwise
    pub fn as_success(&self) -> Option<&TransactionResultSuccess> {
        match *self {
            InnerTransactionResult::Success(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a Success.
    pub fn is_success(&self) -> bool {
        self.as_success().is_some()
    }

    /// If the result is a Failed, returns its value. Returns None otherwise
    pub fn as_failed(&self) -> Option<&TransactionResultFailed> {
        match *self {
            InnerTransactionResult::Failed(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a Failed.
    pub fn is_failed(&self) -> bool {
        self.as_failed().is_some()
    }

    /// If the result is a TooEarly, returns its value. Returns None otherwise
    pub fn as_too_early(&self) -> Option<&TransactionResultTooEarly> {
        match *self {
            InnerTransactionResult::TooEarly(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a TooEarly.
    pub fn is_too_early(&self) -> bool {
        self.as_too_early().is_some()
    }

    /// If the result is a TooLate, returns its value. Returns None otherwise
    pub fn as_too_late(&self) -> Option<&TransactionResultTooLate> {
        match *self {
            InnerTransactionResult::TooLate(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a TooLate.
    pub fn is_too_late(&self) -> bool {
        self.as_too_late().is_some()
    }

    /// If the result is a MissingOperation, returns its value. Returns None otherwise
    pub fn as_missing_operation(&self) -> Option<&TransactionResultMissingOperation> {
        match *self {
            InnerTransactionResult::MissingOperation(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a MissingOperation.
    pub fn is_missing_operation(&self) -> bool {
        self.as_missing_operation().is_some()
    }

    /// If the result is a BadSequence, returns its value. Returns None otherwise
    pub fn as_bad_sequence(&self) -> Option<&TransactionResultBadSequence> {
        match *self {
            InnerTransactionResult::BadSequence(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a BadSequence.
    pub fn is_bad_sequence(&self) -> bool {
        self.as_bad_sequence().is_some()
    }

    /// If the result is a BadAuth, returns its value. Returns None otherwise
    pub fn as_bad_auth(&self) -> Option<&TransactionResultBadAuth> {
        match *self {
            InnerTransactionResult::BadAuth(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a BadAuth.
    pub fn is_bad_auth(&self) -> bool {
        self.as_bad_auth().is_some()
    }

    /// If the result is a InsufficientBalance, returns its value. Returns None otherwise
    pub fn as_insufficient_balance(&self) -> Option<&TransactionResultInsufficientBalance> {
        match *self {
            InnerTransactionResult::InsufficientBalance(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a InsufficientBalance.
    pub fn is_insufficient_balance(&self) -> bool {
        self.as_insufficient_balance().is_some()
    }

    /// If the result is a NoAccount, returns its value. Returns None otherwise
    pub fn as_no_account(&self) -> Option<&TransactionResultNoAccount> {
        match *self {
            InnerTransactionResult::NoAccount(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a NoAccount.
    pub fn is_no_account(&self) -> bool {
        self.as_no_account().is_some()
    }

    /// If the result is a InsufficientFee, returns its value. Returns None otherwise
    pub fn as_insufficient_fee(&self) -> Option<&TransactionResultInsufficientFee> {
        match *self {
            InnerTransactionResult::InsufficientFee(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a InsufficientFee.
    pub fn is_insufficient_fee(&self) -> bool {
        self.as_insufficient_fee().is_some()
    }

    /// If the result is a BadAuthExtra, returns its value. Returns None otherwise
    pub fn as_bad_auth_extra(&self) -> Option<&TransactionResultBadAuthExtra> {
        match *self {
            InnerTransactionResult::BadAuthExtra(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a BadAuthExtra.
    pub fn is_bad_auth_extra(&self) -> bool {
        self.as_bad_auth_extra().is_some()
    }

    /// If the result is a InternalError, returns its value. Returns None otherwise
    pub fn as_internal_error(&self) -> Option<&TransactionResultInternalError> {
        match *self {
            InnerTransactionResult::InternalError(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a InternalError.
    pub fn is_internal_error(&self) -> bool {
        self.as_internal_error().is_some()
    }

    /// If the result is a NotSupported, returns its value. Returns None otherwise
    pub fn as_not_supported(&self) -> Option<&TransactionResultNotSupported> {
        match *self {
            InnerTransactionResult::NotSupported(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a NotSupported.
    pub fn is_not_supported(&self) -> bool {
        self.as_not_supported().is_some()
    }

    /// If the result is a BadSponsorship, returns its value. Returns None otherwise
    pub fn as_bad_sponsorship(&self) -> Option<&TransactionResultBadSponsorship> {
        match *self {
            InnerTransactionResult::BadSponsorship(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the transaction result is a BadSponsorship.
    pub fn is_bad_sponsorship(&self) -> bool {
        self.as_bad_sponsorship().is_some()
    }

    /// Creates `TransactionResult` from xdr object.
    pub fn from_xdr(x: &xdr::InnerTransactionResult) -> Result<InnerTransactionResult> {
        let fee_charged = Stroops::from_xdr_int64(x.fee_charged)?;
        match x.result {
            xdr::InnerTransactionResultResult::TxSuccess(ref xdr_results) => {
                let mut results = Vec::new();
                for xdr_result in xdr_results.as_slice() {
                    let result = OperationResult::from_xdr(xdr_result)?;
                    results.push(result);
                }
                let inner = TransactionResultSuccess {
                    fee_charged,
                    results,
                };
                Ok(InnerTransactionResult::Success(inner))
            }
            xdr::InnerTransactionResultResult::TxFailed(ref xdr_results) => {
                let mut results = Vec::new();
                for xdr_result in xdr_results.as_slice() {
                    let result = OperationResult::from_xdr(xdr_result)?;
                    results.push(result);
                }
                let inner = TransactionResultFailed {
                    fee_charged,
                    results,
                };
                Ok(InnerTransactionResult::Failed(inner))
            }
            xdr::InnerTransactionResultResult::TxTooEarly => {
                let inner = TransactionResultTooEarly { fee_charged };
                Ok(InnerTransactionResult::TooEarly(inner))
            }
            xdr::InnerTransactionResultResult::TxTooLate => {
                let inner = TransactionResultTooLate { fee_charged };
                Ok(InnerTransactionResult::TooLate(inner))
            }
            xdr::InnerTransactionResultResult::TxMissingOperation => {
                let inner = TransactionResultMissingOperation { fee_charged };
                Ok(InnerTransactionResult::MissingOperation(inner))
            }
            xdr::InnerTransactionResultResult::TxBadSeq => {
                let inner = TransactionResultBadSequence { fee_charged };
                Ok(InnerTransactionResult::BadSequence(inner))
            }
            xdr::InnerTransactionResultResult::TxBadAuth => {
                let inner = TransactionResultBadAuth { fee_charged };
                Ok(InnerTransactionResult::BadAuth(inner))
            }
            xdr::InnerTransactionResultResult::TxInsufficientBalance => {
                let inner = TransactionResultInsufficientBalance { fee_charged };
                Ok(InnerTransactionResult::InsufficientBalance(inner))
            }
            xdr::InnerTransactionResultResult::TxNoAccount => {
                let inner = TransactionResultNoAccount { fee_charged };
                Ok(InnerTransactionResult::NoAccount(inner))
            }
            xdr::InnerTransactionResultResult::TxInsufficientFee => {
                let inner = TransactionResultInsufficientFee { fee_charged };
                Ok(InnerTransactionResult::InsufficientFee(inner))
            }
            xdr::InnerTransactionResultResult::TxBadAuthExtra => {
                let inner = TransactionResultBadAuthExtra { fee_charged };
                Ok(InnerTransactionResult::BadAuthExtra(inner))
            }
            xdr::InnerTransactionResultResult::TxInternalError => {
                let inner = TransactionResultInternalError { fee_charged };
                Ok(InnerTransactionResult::InternalError(inner))
            }
            xdr::InnerTransactionResultResult::TxNotSupported => {
                let inner = TransactionResultNotSupported { fee_charged };
                Ok(InnerTransactionResult::NotSupported(inner))
            }
            xdr::InnerTransactionResultResult::TxBadSponsorship => {
                let inner = TransactionResultBadSponsorship { fee_charged };
                Ok(InnerTransactionResult::BadSponsorship(inner))
            }
            xdr::InnerTransactionResultResult::TxBadMinSeqAgeOrGap => {
                let inner = TransactionResultBadMinSeqAgeOrGap { fee_charged };
                Ok(InnerTransactionResult::BadMinSeqAgeOrGap(inner))
            }
            xdr::InnerTransactionResultResult::TxMalformed => {
                let inner = TransactionResultMalformed { fee_charged };
                Ok(InnerTransactionResult::Malformed(inner))
            }
            xdr::InnerTransactionResultResult::TxSorobanInvalid => {
                let inner = TransactionResultSorobanInvalid { fee_charged };
                Ok(InnerTransactionResult::SorobanInvalid(inner))
            }
        }
    }
}

impl xdr::ReadXdr for InnerTransactionResult {
    fn read_xdr<R: Read>(r: &mut xdr::Limited<R>) -> xdr::Result<Self> {
        let xdr_result = xdr::InnerTransactionResult::read_xdr(r)?;
        Self::from_xdr(&xdr_result).map_err(|_| xdr::Error::Invalid)
    }
}

#[cfg(test)]
mod tests {
    use super::TransactionResult;
    use crate::xdr::XDRDeserialize;

    #[test]
    fn test_fee_bump_success() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0AAAAAAAAAAAAAAAAAAAAAA=";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_fee_bump_success());
    }

    #[test]
    fn test_fee_bump_failed() {
        let xdr = "AAAAAAAAA+j////zAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0AAAAAAAAAAAAAAAAAAAAAA=";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_fee_bump_failed());
    }

    #[test]
    fn test_success() {
        let xdr = "AAAAAAAAA+gAAAAAAAAAAAAAAAA=";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_success());
    }

    #[test]
    fn test_failed() {
        let xdr = "AAAAAAAAA+j/////AAAAAAAAAAA=";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_failed());
    }

    #[test]
    fn test_too_early() {
        let xdr = "AAAAAAAPQkD////+AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_too_early());
    }

    #[test]
    fn test_too_late() {
        let xdr = "AAAAAAAPQkD////9AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_too_late());
    }

    #[test]
    fn test_missing_operation() {
        let xdr = "AAAAAAAPQkD////8AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_missing_operation());
    }

    #[test]
    fn test_bad_sequence() {
        let xdr = "AAAAAAAPQkD////7AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_bad_sequence());
    }

    #[test]
    fn test_bad_auth() {
        let xdr = "AAAAAAAPQkD////6AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_bad_auth());
    }

    #[test]
    fn test_insufficient_balance() {
        let xdr = "AAAAAAAPQkD////5AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_insufficient_balance());
    }

    #[test]
    fn test_no_account() {
        let xdr = "AAAAAAAPQkD////4AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_no_account());
    }

    #[test]
    fn test_insufficient_fee() {
        let xdr = "AAAAAAAPQkD////3AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_insufficient_fee());
    }

    #[test]
    fn test_bad_auth_extra() {
        let xdr = "AAAAAAAPQkD////2AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_bad_auth_extra());
    }

    #[test]
    fn test_internal_error() {
        let xdr = "AAAAAAAPQkD////1AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_internal_error());
    }

    #[test]
    fn test_not_supported() {
        let xdr = "AAAAAAAPQkD////0AAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_not_supported());
    }

    #[test]
    fn test_bad_sponsorship() {
        let xdr = "AAAAAAAPQkD////yAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        assert!(result.is_bad_sponsorship());
    }

    #[test]
    fn test_inner_success() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0AAAAAAAAAAAAAAAAAAAAAA=";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_success());
    }

    #[test]
    fn test_inner_failed() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////8AAAAAAAAAAAAAAAA=";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_failed());
    }

    #[test]
    fn test_inner_too_early() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////4AAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_too_early());
    }

    #[test]
    fn test_inner_too_late() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////0AAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_too_late());
    }

    #[test]
    fn test_inner_missing_operation() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////wAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_missing_operation());
    }

    #[test]
    fn test_inner_bad_sequence() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////sAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_bad_sequence());
    }

    #[test]
    fn test_inner_bad_auth() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////oAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_bad_auth());
    }

    #[test]
    fn test_inner_insufficient_balance() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////kAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_insufficient_balance());
    }

    #[test]
    fn test_inner_no_account() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////gAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_no_account());
    }

    #[test]
    fn test_inner_insufficient_fee() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////cAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_insufficient_fee());
    }

    #[test]
    fn test_inner_bad_auth_extra() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////YAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_bad_auth_extra());
    }

    #[test]
    fn test_inner_internal_error() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////UAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_internal_error());
    }

    #[test]
    fn test_inner_not_supported() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////QAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_not_supported());
    }

    #[test]
    fn test_inner_bad_sponsorship() {
        let xdr = "AAAAAAAAA+gAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH0P////IAAAAAAAAAAA==";
        let result = TransactionResult::from_xdr_base64(xdr).unwrap();
        let inner_result = result.as_fee_bump_success().unwrap().result.clone();
        assert!(inner_result.is_bad_sponsorship());
    }
}
