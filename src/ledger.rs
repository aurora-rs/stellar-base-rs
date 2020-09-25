use crate::asset::Asset;
use crate::claim::ClaimableBalanceId;
use crate::crypto::PublicKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerKey {
    Account(PublicKey),
    Trustline(PublicKey, Asset),
    Offer(PublicKey, i64),
    Data(PublicKey, String),
    ClaimableBalance(ClaimableBalanceId),
}
