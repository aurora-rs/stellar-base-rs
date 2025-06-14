use crate::asset::TrustLineAsset;
use crate::claim::ClaimableBalanceId;
use crate::crypto::PublicKey;
use crate::liquidity_pool::LiquidityPoolId;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerKey {
    Account(PublicKey),
    Trustline(PublicKey, TrustLineAsset),
    Offer(PublicKey, i64),
    Data(PublicKey, String),
    ClaimableBalance(ClaimableBalanceId),
    LiquidityPool(LiquidityPoolId),
    ContractData(xdr::LedgerKeyContractData),
    ContractCode(xdr::LedgerKeyContractCode),
    ConfigSetting(xdr::LedgerKeyConfigSetting),
    Ttl(xdr::LedgerKeyTtl),
}
