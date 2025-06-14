use crate::amount::{Price, Stroops};
use crate::asset::Asset;
use crate::claim::ClaimableBalanceId;
use crate::crypto::PublicKey;
use crate::error::Result;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationResult {
    Inner(InnerOperationResult),
    BadAuth,
    NoAccount,
    NotSupported,
    TooManySubentries,
    ExceededWorkLimit,
    TooManySponsoring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InnerOperationResult {
    CreateAccount(CreateAccountResult),
    Payment(PaymentResult),
    PathPaymentStrictReceive(PathPaymentStrictReceiveResult),
    PathPaymentStrictSend(PathPaymentStrictSendResult),
    ManageSellOffer(ManageSellOfferResult),
    ManageBuyOffer(ManageBuyOfferResult),
    CreatePassiveSellOffer(ManageSellOfferResult),
    SetOptions(SetOptionsResult),
    ChangeTrust(ChangeTrustResult),
    AllowTrust(AllowTrustResult),
    AccountMerge(AccountMergeResult),
    Inflation(InflationResult),
    ManageData(ManageDataResult),
    BumpSequence(BumpSequenceResult),
    CreateClaimableBalance(CreateClaimableBalanceResult),
    ClaimClaimableBalance(ClaimClaimableBalanceResult),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesResult),
    EndSponsoringFutureReserves(EndSponsoringFutureReservesResult),
    RevokeSponsorship(RevokeSponsorshipResult),
    Clawback(ClawbackResult),
    ClawbackClaimableBalance(ClawbackClaimableBalanceResult),
    SetTrustLineFlags(SetTrustLineFlagsResult),
    LiquidityPoolDeposit(LiquidityPoolDepositResult),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawResult),
    InvokeHostFunction(InvokeHostFunctionResult),
    ExtendFootprintTtl(ExtendFootprintTtlResult),
    RestoreFootprint(RestoreFootprintResult),
}

pub use xdr::CreateAccountResult;
pub use xdr::PaymentResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathPaymentStrictReceiveResult {
    Success(PathPaymentStrictReceiveResultSuccess),
    NoIssuer(Asset),
    Malformed,
    Underfunded,
    SourceNoTrust,
    SourceNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    TooFewOffers,
    OfferCrossSelf,
    OverSendMax,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathPaymentStrictSendResult {
    Success(PathPaymentStrictSendResultSuccess),
    NoIssuer(Asset),
    Malformed,
    Underfunded,
    SourceNoTrust,
    SourceNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    TooFewOffers,
    OfferCrossSelf,
    UnderDestinationMin,
}

pub use xdr::SetOptionsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManageSellOfferResult {
    Success(ManageOfferResultSuccess),
    Malformed,
    SellNoTrust,
    BuyNoTrust,
    SellNotAuthorized,
    BuyNotAuthorized,
    LineFull,
    Underfunded,
    CrossSelf,
    SellNoIssuer,
    BuyNoIssuer,
    NotFound,
    LowReserve,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManageBuyOfferResult {
    Success(ManageOfferResultSuccess),
    Malformed,
    SellNoTrust,
    BuyNoTrust,
    SellNotAuthorized,
    BuyNotAuthorized,
    LineFull,
    Underfunded,
    CrossSelf,
    SellNoIssuer,
    BuyNoIssuer,
    NotFound,
    LowReserve,
}

pub use xdr::AllowTrustResult;
pub use xdr::ChangeTrustResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountMergeResult {
    Success(Stroops),
    Malformed,
    NoAccount,
    ImmutableSet,
    HasSubEntries,
    SequenceTooFar,
    DestinationFull,
    IsSponsor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InflationResult {
    Success(Vec<InflationPayout>),
    NotTime,
}

pub use xdr::BumpSequenceResult;
pub use xdr::ManageDataResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateClaimableBalanceResult {
    Success(ClaimableBalanceId),
    Malformed,
    LowReserve,
    NoTrust,
    NotAuthorized,
    Underfunded,
}

pub use xdr::BeginSponsoringFutureReservesResult;
pub use xdr::ClaimClaimableBalanceResult;
pub use xdr::ClawbackClaimableBalanceResult;
pub use xdr::ClawbackResult;
pub use xdr::EndSponsoringFutureReservesResult;
pub use xdr::ExtendFootprintTtlResult;
pub use xdr::InvokeHostFunctionResult;
pub use xdr::LiquidityPoolDepositResult;
pub use xdr::LiquidityPoolWithdrawResult;
pub use xdr::RestoreFootprintResult;
pub use xdr::RevokeSponsorshipResult;
pub use xdr::SetTrustLineFlagsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPaymentStrictReceiveResultSuccess {
    pub offers: Vec<ClaimAtom>,
    pub last: SimplePaymentResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPaymentStrictSendResultSuccess {
    pub offers: Vec<ClaimAtom>,
    pub last: SimplePaymentResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManageOfferResultSuccess {
    pub offers_claimed: Vec<ClaimAtom>,
    pub offer: OfferResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimplePaymentResult {
    pub destination: PublicKey,
    pub asset: Asset,
    pub amount: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimAtom {
    OrderBook(ClaimOfferAtom),
    LiquidityPool(ClaimLiquidityAtom),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimLiquidityAtom {
    pub pool_id: Vec<u8>,
    pub asset_sold: Asset,
    pub amount_sold: Stroops,
    pub asset_bought: Asset,
    pub amount_bought: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimOfferAtom {
    pub seller_id: PublicKey,
    pub offer_id: i64,
    pub asset_sold: Asset,
    pub amount_sold: Stroops,
    pub asset_bought: Asset,
    pub amount_bought: Stroops,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OfferResult {
    Created(OfferEntry),
    Updated(OfferEntry),
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfferEntry {
    pub seller_id: PublicKey,
    pub offer_id: i64,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: Stroops,
    pub price: Price,
    pub flags: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InflationPayout {
    pub destination: PublicKey,
    pub amount: Stroops,
}

impl OperationResult {
    pub fn from_xdr(x: &xdr::OperationResult) -> Result<OperationResult> {
        match *x {
            xdr::OperationResult::OpInner(ref xdr_inner) => {
                let inner = InnerOperationResult::from_xdr(xdr_inner)?;
                Ok(OperationResult::Inner(inner))
            }
            xdr::OperationResult::OpBadAuth => Ok(OperationResult::BadAuth),
            xdr::OperationResult::OpNoAccount => Ok(OperationResult::NoAccount),
            xdr::OperationResult::OpNotSupported => Ok(OperationResult::NotSupported),
            xdr::OperationResult::OpTooManySubentries => Ok(OperationResult::TooManySubentries),
            xdr::OperationResult::OpExceededWorkLimit => Ok(OperationResult::ExceededWorkLimit),
            xdr::OperationResult::OpTooManySponsoring => Ok(OperationResult::TooManySponsoring),
        }
    }
}

impl InnerOperationResult {
    pub fn from_xdr(x: &xdr::OperationResultTr) -> Result<InnerOperationResult> {
        match x {
            xdr::OperationResultTr::CreateAccount(xdr_inner) => {
                Ok(InnerOperationResult::CreateAccount(xdr_inner.clone()))
            }
            xdr::OperationResultTr::Payment(xdr_inner) => {
                Ok(InnerOperationResult::Payment(xdr_inner.clone()))
            }
            xdr::OperationResultTr::PathPaymentStrictReceive(ref xdr_inner) => {
                let inner = PathPaymentStrictReceiveResult::from_xdr(xdr_inner)?;
                Ok(InnerOperationResult::PathPaymentStrictReceive(inner))
            }
            xdr::OperationResultTr::ManageSellOffer(ref xdr_inner) => {
                let inner = ManageSellOfferResult::from_xdr(xdr_inner)?;
                Ok(InnerOperationResult::ManageSellOffer(inner))
            }
            xdr::OperationResultTr::CreatePassiveSellOffer(ref xdr_inner) => {
                let inner = ManageSellOfferResult::from_xdr(xdr_inner)?;
                Ok(InnerOperationResult::CreatePassiveSellOffer(inner))
            }
            xdr::OperationResultTr::SetOptions(xdr_inner) => {
                Ok(InnerOperationResult::SetOptions(xdr_inner.clone()))
            }
            xdr::OperationResultTr::ChangeTrust(xdr_inner) => {
                Ok(InnerOperationResult::ChangeTrust(xdr_inner.clone()))
            }
            xdr::OperationResultTr::AllowTrust(xdr_inner) => {
                Ok(InnerOperationResult::AllowTrust(xdr_inner.clone()))
            }
            xdr::OperationResultTr::AccountMerge(ref xdr_inner) => {
                let inner = AccountMergeResult::from_xdr(xdr_inner)?;
                Ok(InnerOperationResult::AccountMerge(inner))
            }
            xdr::OperationResultTr::Inflation(ref xdr_inner) => {
                let inner = InflationResult::from_xdr(xdr_inner)?;
                Ok(InnerOperationResult::Inflation(inner))
            }
            xdr::OperationResultTr::ManageData(xdr_inner) => {
                Ok(InnerOperationResult::ManageData(xdr_inner.clone()))
            }
            xdr::OperationResultTr::BumpSequence(xdr_inner) => {
                Ok(InnerOperationResult::BumpSequence(xdr_inner.clone()))
            }
            xdr::OperationResultTr::ManageBuyOffer(ref xdr_inner) => {
                let inner = ManageBuyOfferResult::from_xdr(xdr_inner)?;
                Ok(InnerOperationResult::ManageBuyOffer(inner))
            }
            xdr::OperationResultTr::PathPaymentStrictSend(ref xdr_inner) => {
                let inner = PathPaymentStrictSendResult::from_xdr(xdr_inner)?;
                Ok(InnerOperationResult::PathPaymentStrictSend(inner))
            }
            xdr::OperationResultTr::CreateClaimableBalance(ref xdr_inner) => {
                let inner = CreateClaimableBalanceResult::from_xdr(xdr_inner)?;
                Ok(InnerOperationResult::CreateClaimableBalance(inner))
            }
            xdr::OperationResultTr::ClaimClaimableBalance(xdr_inner) => Ok(
                InnerOperationResult::ClaimClaimableBalance(xdr_inner.clone()),
            ),
            xdr::OperationResultTr::BeginSponsoringFutureReserves(xdr_inner) => Ok(
                InnerOperationResult::BeginSponsoringFutureReserves(xdr_inner.clone()),
            ),
            xdr::OperationResultTr::EndSponsoringFutureReserves(xdr_inner) => Ok(
                InnerOperationResult::EndSponsoringFutureReserves(xdr_inner.clone()),
            ),
            xdr::OperationResultTr::RevokeSponsorship(xdr_inner) => {
                Ok(InnerOperationResult::RevokeSponsorship(xdr_inner.clone()))
            }
            xdr::OperationResultTr::Clawback(xdr_inner) => {
                Ok(InnerOperationResult::Clawback(xdr_inner.clone()))
            }
            xdr::OperationResultTr::ClawbackClaimableBalance(xdr_inner) => Ok(
                InnerOperationResult::ClawbackClaimableBalance(xdr_inner.clone()),
            ),
            xdr::OperationResultTr::SetTrustLineFlags(xdr_inner) => {
                Ok(InnerOperationResult::SetTrustLineFlags(xdr_inner.clone()))
            }
            xdr::OperationResultTr::LiquidityPoolDeposit(xdr_inner) => Ok(
                InnerOperationResult::LiquidityPoolDeposit(xdr_inner.clone()),
            ),
            xdr::OperationResultTr::LiquidityPoolWithdraw(xdr_inner) => Ok(
                InnerOperationResult::LiquidityPoolWithdraw(xdr_inner.clone()),
            ),
            xdr::OperationResultTr::InvokeHostFunction(xdr_inner) => {
                Ok(InnerOperationResult::InvokeHostFunction(xdr_inner.clone()))
            }
            xdr::OperationResultTr::ExtendFootprintTtl(xdr_inner) => {
                Ok(InnerOperationResult::ExtendFootprintTtl(xdr_inner.clone()))
            }
            xdr::OperationResultTr::RestoreFootprint(xdr_inner) => {
                Ok(InnerOperationResult::RestoreFootprint(xdr_inner.clone()))
            }
        }
    }
}

impl PathPaymentStrictReceiveResult {
    pub fn from_xdr(
        x: &xdr::PathPaymentStrictReceiveResult,
    ) -> Result<PathPaymentStrictReceiveResult> {
        match *x {
            xdr::PathPaymentStrictReceiveResult::Success(ref xdr_inner) => {
                let inner = PathPaymentStrictReceiveResultSuccess::from_xdr(xdr_inner)?;
                Ok(PathPaymentStrictReceiveResult::Success(inner))
            }
            xdr::PathPaymentStrictReceiveResult::Malformed => {
                Ok(PathPaymentStrictReceiveResult::Malformed)
            }
            xdr::PathPaymentStrictReceiveResult::Underfunded => {
                Ok(PathPaymentStrictReceiveResult::Underfunded)
            }
            xdr::PathPaymentStrictReceiveResult::SrcNoTrust => {
                Ok(PathPaymentStrictReceiveResult::SourceNoTrust)
            }
            xdr::PathPaymentStrictReceiveResult::SrcNotAuthorized => {
                Ok(PathPaymentStrictReceiveResult::SourceNotAuthorized)
            }
            xdr::PathPaymentStrictReceiveResult::NoDestination => {
                Ok(PathPaymentStrictReceiveResult::NoDestination)
            }
            xdr::PathPaymentStrictReceiveResult::NoTrust => {
                Ok(PathPaymentStrictReceiveResult::NoTrust)
            }
            xdr::PathPaymentStrictReceiveResult::NotAuthorized => {
                Ok(PathPaymentStrictReceiveResult::NotAuthorized)
            }
            xdr::PathPaymentStrictReceiveResult::NoIssuer(ref xdr_asset) => {
                let asset = Asset::from_xdr(xdr_asset)?;
                Ok(PathPaymentStrictReceiveResult::NoIssuer(asset))
            }
            xdr::PathPaymentStrictReceiveResult::LineFull => {
                Ok(PathPaymentStrictReceiveResult::LineFull)
            }
            xdr::PathPaymentStrictReceiveResult::TooFewOffers => {
                Ok(PathPaymentStrictReceiveResult::TooFewOffers)
            }
            xdr::PathPaymentStrictReceiveResult::OfferCrossSelf => {
                Ok(PathPaymentStrictReceiveResult::OfferCrossSelf)
            }
            xdr::PathPaymentStrictReceiveResult::OverSendmax => {
                Ok(PathPaymentStrictReceiveResult::OverSendMax)
            }
        }
    }
}

impl PathPaymentStrictSendResult {
    pub fn from_xdr(x: &xdr::PathPaymentStrictSendResult) -> Result<PathPaymentStrictSendResult> {
        match *x {
            xdr::PathPaymentStrictSendResult::Success(ref xdr_inner) => {
                let inner = PathPaymentStrictSendResultSuccess::from_xdr(xdr_inner)?;
                Ok(PathPaymentStrictSendResult::Success(inner))
            }
            xdr::PathPaymentStrictSendResult::Malformed => {
                Ok(PathPaymentStrictSendResult::Malformed)
            }
            xdr::PathPaymentStrictSendResult::Underfunded => {
                Ok(PathPaymentStrictSendResult::Underfunded)
            }
            xdr::PathPaymentStrictSendResult::SrcNoTrust => {
                Ok(PathPaymentStrictSendResult::SourceNoTrust)
            }
            xdr::PathPaymentStrictSendResult::SrcNotAuthorized => {
                Ok(PathPaymentStrictSendResult::SourceNotAuthorized)
            }
            xdr::PathPaymentStrictSendResult::NoDestination => {
                Ok(PathPaymentStrictSendResult::NoDestination)
            }
            xdr::PathPaymentStrictSendResult::NoTrust => Ok(PathPaymentStrictSendResult::NoTrust),
            xdr::PathPaymentStrictSendResult::NotAuthorized => {
                Ok(PathPaymentStrictSendResult::NotAuthorized)
            }
            xdr::PathPaymentStrictSendResult::LineFull => Ok(PathPaymentStrictSendResult::LineFull),
            xdr::PathPaymentStrictSendResult::NoIssuer(ref xdr_asset) => {
                let asset = Asset::from_xdr(xdr_asset)?;
                Ok(PathPaymentStrictSendResult::NoIssuer(asset))
            }
            xdr::PathPaymentStrictSendResult::TooFewOffers => {
                Ok(PathPaymentStrictSendResult::TooFewOffers)
            }
            xdr::PathPaymentStrictSendResult::OfferCrossSelf => {
                Ok(PathPaymentStrictSendResult::OfferCrossSelf)
            }
            xdr::PathPaymentStrictSendResult::UnderDestmin => {
                Ok(PathPaymentStrictSendResult::UnderDestinationMin)
            }
        }
    }
}

impl ManageSellOfferResult {
    pub fn from_xdr(x: &xdr::ManageSellOfferResult) -> Result<ManageSellOfferResult> {
        match *x {
            xdr::ManageSellOfferResult::Success(ref xdr_inner) => {
                let inner = ManageOfferResultSuccess::from_xdr(xdr_inner)?;
                Ok(ManageSellOfferResult::Success(inner))
            }
            xdr::ManageSellOfferResult::Malformed => Ok(ManageSellOfferResult::Malformed),
            xdr::ManageSellOfferResult::SellNoTrust => Ok(ManageSellOfferResult::SellNoTrust),
            xdr::ManageSellOfferResult::BuyNoTrust => Ok(ManageSellOfferResult::BuyNoTrust),
            xdr::ManageSellOfferResult::SellNotAuthorized => {
                Ok(ManageSellOfferResult::SellNotAuthorized)
            }
            xdr::ManageSellOfferResult::BuyNotAuthorized => {
                Ok(ManageSellOfferResult::BuyNotAuthorized)
            }
            xdr::ManageSellOfferResult::LineFull => Ok(ManageSellOfferResult::LineFull),
            xdr::ManageSellOfferResult::Underfunded => Ok(ManageSellOfferResult::Underfunded),
            xdr::ManageSellOfferResult::CrossSelf => Ok(ManageSellOfferResult::CrossSelf),
            xdr::ManageSellOfferResult::SellNoIssuer => Ok(ManageSellOfferResult::SellNoIssuer),
            xdr::ManageSellOfferResult::BuyNoIssuer => Ok(ManageSellOfferResult::BuyNoIssuer),
            xdr::ManageSellOfferResult::NotFound => Ok(ManageSellOfferResult::NotFound),
            xdr::ManageSellOfferResult::LowReserve => Ok(ManageSellOfferResult::LowReserve),
        }
    }
}

impl ManageBuyOfferResult {
    pub fn from_xdr(x: &xdr::ManageBuyOfferResult) -> Result<ManageBuyOfferResult> {
        match *x {
            xdr::ManageBuyOfferResult::Success(ref xdr_inner) => {
                let inner = ManageOfferResultSuccess::from_xdr(xdr_inner)?;
                Ok(ManageBuyOfferResult::Success(inner))
            }
            xdr::ManageBuyOfferResult::Malformed => Ok(ManageBuyOfferResult::Malformed),
            xdr::ManageBuyOfferResult::SellNoTrust => Ok(ManageBuyOfferResult::SellNoTrust),
            xdr::ManageBuyOfferResult::BuyNoTrust => Ok(ManageBuyOfferResult::BuyNoTrust),
            xdr::ManageBuyOfferResult::SellNotAuthorized => {
                Ok(ManageBuyOfferResult::SellNotAuthorized)
            }
            xdr::ManageBuyOfferResult::BuyNotAuthorized => {
                Ok(ManageBuyOfferResult::BuyNotAuthorized)
            }
            xdr::ManageBuyOfferResult::LineFull => Ok(ManageBuyOfferResult::LineFull),
            xdr::ManageBuyOfferResult::Underfunded => Ok(ManageBuyOfferResult::Underfunded),
            xdr::ManageBuyOfferResult::CrossSelf => Ok(ManageBuyOfferResult::CrossSelf),
            xdr::ManageBuyOfferResult::BuyNoIssuer => Ok(ManageBuyOfferResult::BuyNoIssuer),
            xdr::ManageBuyOfferResult::SellNoIssuer => Ok(ManageBuyOfferResult::SellNoIssuer),
            xdr::ManageBuyOfferResult::NotFound => Ok(ManageBuyOfferResult::NotFound),
            xdr::ManageBuyOfferResult::LowReserve => Ok(ManageBuyOfferResult::LowReserve),
        }
    }
}

impl AccountMergeResult {
    pub fn from_xdr(x: &xdr::AccountMergeResult) -> Result<AccountMergeResult> {
        match *x {
            xdr::AccountMergeResult::Success(xdr_balance) => {
                let balance = Stroops::from_xdr_int64(xdr_balance)?;
                Ok(AccountMergeResult::Success(balance))
            }
            xdr::AccountMergeResult::Malformed => Ok(AccountMergeResult::Malformed),
            xdr::AccountMergeResult::NoAccount => Ok(AccountMergeResult::NoAccount),
            xdr::AccountMergeResult::ImmutableSet => Ok(AccountMergeResult::ImmutableSet),
            xdr::AccountMergeResult::HasSubEntries => Ok(AccountMergeResult::HasSubEntries),
            xdr::AccountMergeResult::SeqnumTooFar => Ok(AccountMergeResult::SequenceTooFar),
            xdr::AccountMergeResult::DestFull => Ok(AccountMergeResult::DestinationFull),
            xdr::AccountMergeResult::IsSponsor => Ok(AccountMergeResult::IsSponsor),
        }
    }
}

impl InflationResult {
    pub fn from_xdr(x: &xdr::InflationResult) -> Result<InflationResult> {
        match *x {
            xdr::InflationResult::Success(ref xdr_payouts) => {
                let mut payouts = Vec::new();
                for xdr_payout in xdr_payouts.as_slice() {
                    let payout = InflationPayout::from_xdr(xdr_payout)?;
                    payouts.push(payout);
                }
                Ok(InflationResult::Success(payouts))
            }
            xdr::InflationResult::NotTime => Ok(InflationResult::NotTime),
        }
    }
}

impl CreateClaimableBalanceResult {
    pub fn from_xdr(x: &xdr::CreateClaimableBalanceResult) -> Result<CreateClaimableBalanceResult> {
        match *x {
            xdr::CreateClaimableBalanceResult::Success(ref xdr_id) => {
                let id = ClaimableBalanceId::from_xdr(xdr_id)?;
                Ok(CreateClaimableBalanceResult::Success(id))
            }
            xdr::CreateClaimableBalanceResult::Malformed => {
                Ok(CreateClaimableBalanceResult::Malformed)
            }
            xdr::CreateClaimableBalanceResult::LowReserve => {
                Ok(CreateClaimableBalanceResult::LowReserve)
            }
            xdr::CreateClaimableBalanceResult::NoTrust => Ok(CreateClaimableBalanceResult::NoTrust),
            xdr::CreateClaimableBalanceResult::NotAuthorized => {
                Ok(CreateClaimableBalanceResult::NotAuthorized)
            }
            xdr::CreateClaimableBalanceResult::Underfunded => {
                Ok(CreateClaimableBalanceResult::Underfunded)
            }
        }
    }
}

impl PathPaymentStrictReceiveResultSuccess {
    pub fn from_xdr(
        x: &xdr::PathPaymentStrictReceiveResultSuccess,
    ) -> Result<PathPaymentStrictReceiveResultSuccess> {
        let mut offers = Vec::new();
        for xdr_offer in x.offers.as_slice() {
            let offer = ClaimAtom::from_xdr(xdr_offer)?;
            offers.push(offer);
        }
        let last = SimplePaymentResult::from_xdr(&x.last)?;
        Ok(PathPaymentStrictReceiveResultSuccess { last, offers })
    }
}

impl PathPaymentStrictSendResultSuccess {
    pub fn from_xdr(
        x: &xdr::PathPaymentStrictSendResultSuccess,
    ) -> Result<PathPaymentStrictSendResultSuccess> {
        let mut offers = Vec::new();
        for xdr_offer in x.offers.as_slice() {
            let offer = ClaimAtom::from_xdr(xdr_offer)?;
            offers.push(offer);
        }
        let last = SimplePaymentResult::from_xdr(&x.last)?;
        Ok(PathPaymentStrictSendResultSuccess { last, offers })
    }
}

impl ManageOfferResultSuccess {
    pub fn from_xdr(x: &xdr::ManageOfferSuccessResult) -> Result<ManageOfferResultSuccess> {
        let mut offers_claimed = Vec::new();
        for xdr_offer in x.offers_claimed.as_slice() {
            let offer = ClaimAtom::from_xdr(xdr_offer)?;
            offers_claimed.push(offer);
        }
        let offer = OfferResult::from_xdr(&x.offer)?;
        Ok(ManageOfferResultSuccess {
            offers_claimed,
            offer,
        })
    }
}

impl SimplePaymentResult {
    pub fn from_xdr(x: &xdr::SimplePaymentResult) -> Result<SimplePaymentResult> {
        let destination = PublicKey::from_xdr_account_id(&x.destination)?;
        let asset = Asset::from_xdr(&x.asset)?;
        let amount = Stroops::from_xdr_int64(x.amount)?;
        Ok(SimplePaymentResult {
            destination,
            asset,
            amount,
        })
    }
}

impl ClaimAtom {
    pub fn from_xdr(x: &xdr::ClaimAtom) -> Result<ClaimAtom> {
        Ok(match *x {
            xdr::ClaimAtom::V0(ref inner_xdr) => {
                let inner = ClaimOfferAtom::from_xdr_v0(inner_xdr)?;
                ClaimAtom::OrderBook(inner)
            }
            xdr::ClaimAtom::OrderBook(ref inner_xdr) => {
                let inner = ClaimOfferAtom::from_xdr(inner_xdr)?;
                ClaimAtom::OrderBook(inner)
            }
            xdr::ClaimAtom::LiquidityPool(ref inner_xdr) => {
                let inner = ClaimLiquidityAtom::from_xdr(inner_xdr)?;
                ClaimAtom::LiquidityPool(inner)
            }
        })
    }
}

impl ClaimOfferAtom {
    pub fn from_xdr(x: &xdr::ClaimOfferAtom) -> Result<ClaimOfferAtom> {
        let seller_id = PublicKey::from_xdr_account_id(&x.seller_id)?;
        let offer_id = x.offer_id;
        let asset_sold = Asset::from_xdr(&x.asset_sold)?;
        let amount_sold = Stroops::from_xdr_int64(x.amount_sold)?;
        let asset_bought = Asset::from_xdr(&x.asset_bought)?;
        let amount_bought = Stroops::from_xdr_int64(x.amount_bought)?;
        Ok(ClaimOfferAtom {
            seller_id,
            offer_id,
            asset_sold,
            amount_sold,
            asset_bought,
            amount_bought,
        })
    }

    pub fn from_xdr_v0(x: &xdr::ClaimOfferAtomV0) -> Result<ClaimOfferAtom> {
        let seller_id = PublicKey::from_slice(x.seller_ed25519.as_slice())?;
        let offer_id = x.offer_id;
        let asset_sold = Asset::from_xdr(&x.asset_sold)?;
        let amount_sold = Stroops::from_xdr_int64(x.amount_sold)?;
        let asset_bought = Asset::from_xdr(&x.asset_bought)?;
        let amount_bought = Stroops::from_xdr_int64(x.amount_bought)?;
        Ok(ClaimOfferAtom {
            seller_id,
            offer_id,
            asset_sold,
            amount_sold,
            asset_bought,
            amount_bought,
        })
    }
}

impl ClaimLiquidityAtom {
    pub fn from_xdr(x: &xdr::ClaimLiquidityAtom) -> Result<ClaimLiquidityAtom> {
        let pool_id = x.liquidity_pool_id.0 .0.to_vec();
        let asset_sold = Asset::from_xdr(&x.asset_sold)?;
        let amount_sold = Stroops::from_xdr_int64(x.amount_sold)?;
        let asset_bought = Asset::from_xdr(&x.asset_bought)?;
        let amount_bought = Stroops::from_xdr_int64(x.amount_bought)?;
        Ok(ClaimLiquidityAtom {
            pool_id,
            asset_sold,
            amount_sold,
            asset_bought,
            amount_bought,
        })
    }
}

impl OfferResult {
    pub fn from_xdr(x: &xdr::ManageOfferSuccessResultOffer) -> Result<OfferResult> {
        match *x {
            xdr::ManageOfferSuccessResultOffer::Created(ref xdr_inner) => {
                let inner = OfferEntry::from_xdr(xdr_inner)?;
                Ok(OfferResult::Created(inner))
            }
            xdr::ManageOfferSuccessResultOffer::Updated(ref xdr_inner) => {
                let inner = OfferEntry::from_xdr(xdr_inner)?;
                Ok(OfferResult::Updated(inner))
            }
            xdr::ManageOfferSuccessResultOffer::Deleted => Ok(OfferResult::Deleted),
        }
    }
}

impl OfferEntry {
    pub fn from_xdr(x: &xdr::OfferEntry) -> Result<OfferEntry> {
        let seller_id = PublicKey::from_xdr_account_id(&x.seller_id)?;
        let offer_id = x.offer_id;
        let selling = Asset::from_xdr(&x.selling)?;
        let buying = Asset::from_xdr(&x.buying)?;
        let amount = Stroops::from_xdr_int64(x.amount)?;
        let price = Price::from_xdr(&x.price)?;
        let flags = x.flags;
        Ok(OfferEntry {
            seller_id,
            offer_id,
            selling,
            buying,
            amount,
            price,
            flags,
        })
    }
}

impl InflationPayout {
    pub fn from_xdr(x: &xdr::InflationPayout) -> Result<InflationPayout> {
        let destination = PublicKey::from_xdr_account_id(&x.destination)?;
        let amount = Stroops::from_xdr_int64(x.amount)?;
        Ok(InflationPayout {
            destination,
            amount,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transaction_result::TransactionResult, xdr::XDRDeserialize as _};

    macro_rules! impl_inner_op_result_test {
        ($test_name:ident, $xdr_str:tt, $expected_res:pat) => {
            #[test]
            fn $test_name() {
                let xdr = $xdr_str;
                let result = TransactionResult::from_xdr_base64(&xdr)
                    .unwrap()
                    .as_failed()
                    .unwrap()
                    .clone();
                assert_eq!(result.results.len(), 1);
                match result.results[0] {
                    OperationResult::Inner(ref inner) => match inner {
                        $expected_res => {}
                        _ => panic!("expected {:?}", stringify!($expected_res)),
                    },
                    _ => panic!("expected OperationResult::Inner"),
                }
            }
        };
    }

    #[test]
    fn test_bad_auth() {
        let xdr = "AAAAAACYloD/////AAAAAf////8AAAAA";
        let result = TransactionResult::from_xdr_base64(xdr)
            .unwrap()
            .as_failed()
            .unwrap()
            .clone();
        assert_eq!(result.results.len(), 1);
        match result.results[0] {
            OperationResult::BadAuth => {}
            _ => panic!("expected OperationResult::BadAuth"),
        }
    }

    #[test]
    fn test_no_account() {
        let xdr = "AAAAAACYloD/////AAAAAf////4AAAAA";
        let result = TransactionResult::from_xdr_base64(xdr)
            .unwrap()
            .as_failed()
            .unwrap()
            .clone();
        assert_eq!(result.results.len(), 1);
        match result.results[0] {
            OperationResult::NoAccount => {}
            _ => panic!("expected OperationResult::NoAccount"),
        }
    }

    #[test]
    fn test_not_supported() {
        let xdr = "AAAAAACYloD/////AAAAAf////0AAAAA";
        let result = TransactionResult::from_xdr_base64(xdr)
            .unwrap()
            .as_failed()
            .unwrap()
            .clone();
        assert_eq!(result.results.len(), 1);
        match result.results[0] {
            OperationResult::NotSupported => {}
            _ => panic!("expected OperationResult::NotSupported"),
        }
    }

    #[test]
    fn test_too_many_subentries() {
        let xdr = "AAAAAAAAA+j/////AAAAAf////wAAAAA";
        let result = TransactionResult::from_xdr_base64(xdr)
            .unwrap()
            .as_failed()
            .unwrap()
            .clone();
        assert_eq!(result.results.len(), 1);
        match result.results[0] {
            OperationResult::TooManySubentries => {}
            _ => panic!("expected OperationResult::TooManySubentries"),
        }
    }

    #[test]
    fn test_exceeded_work_limit() {
        let xdr = "AAAAAAAAA+j/////AAAAAf////sAAAAA";
        let result = TransactionResult::from_xdr_base64(xdr)
            .unwrap()
            .as_failed()
            .unwrap()
            .clone();
        assert_eq!(result.results.len(), 1);
        match result.results[0] {
            OperationResult::ExceededWorkLimit => {}
            _ => panic!("expected OperationResult::ExceededWorkLimit"),
        }
    }

    #[test]
    fn test_too_many_sponsoring() {
        let xdr = "AAAAAAAAA+j/////AAAAAf////oAAAAA";
        let result = TransactionResult::from_xdr_base64(xdr)
            .unwrap()
            .as_failed()
            .unwrap()
            .clone();
        assert_eq!(result.results.len(), 1);
        match result.results[0] {
            OperationResult::TooManySponsoring => {}
            _ => panic!("expected OperationResult::TooManySponsoring"),
        }
    }

    //
    // Create Account Result
    //

    impl_inner_op_result_test!(
        test_create_account_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAAAAAAAAAAAAA=",
        InnerOperationResult::CreateAccount(CreateAccountResult::Success)
    );

    impl_inner_op_result_test!(
        test_create_account_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAA/////wAAAAA=",
        InnerOperationResult::CreateAccount(CreateAccountResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_create_account_underfunded,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAA/////gAAAAA=",
        InnerOperationResult::CreateAccount(CreateAccountResult::Underfunded)
    );

    impl_inner_op_result_test!(
        test_create_account_low_reserve,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAA/////QAAAAA=",
        InnerOperationResult::CreateAccount(CreateAccountResult::LowReserve)
    );

    impl_inner_op_result_test!(
        test_create_account_already_exist,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAA/////AAAAAA=",
        InnerOperationResult::CreateAccount(CreateAccountResult::AlreadyExist)
    );

    //
    // Payment Result
    //

    impl_inner_op_result_test!(
        test_payment_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAABAAAAAAAAAAA=",
        InnerOperationResult::Payment(PaymentResult::Success)
    );

    impl_inner_op_result_test!(
        test_payment_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB/////wAAAAA=",
        InnerOperationResult::Payment(PaymentResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_payment_underfunded,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB/////gAAAAA=",
        InnerOperationResult::Payment(PaymentResult::Underfunded)
    );

    impl_inner_op_result_test!(
        test_payment_source_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB/////QAAAAA=",
        InnerOperationResult::Payment(PaymentResult::SrcNoTrust)
    );

    impl_inner_op_result_test!(
        test_payment_source_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB/////AAAAAA=",
        InnerOperationResult::Payment(PaymentResult::SrcNotAuthorized)
    );

    impl_inner_op_result_test!(
        test_payment_no_destination,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB////+wAAAAA=",
        InnerOperationResult::Payment(PaymentResult::NoDestination)
    );

    impl_inner_op_result_test!(
        test_payment_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB////+gAAAAA=",
        InnerOperationResult::Payment(PaymentResult::NoTrust)
    );

    impl_inner_op_result_test!(
        test_payment_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB////+QAAAAA=",
        InnerOperationResult::Payment(PaymentResult::NotAuthorized)
    );

    impl_inner_op_result_test!(
        test_payment_line_full,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB////+AAAAAA=",
        InnerOperationResult::Payment(PaymentResult::LineFull)
    );

    impl_inner_op_result_test!(
        test_payment_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAB////9wAAAAA=",
        InnerOperationResult::Payment(PaymentResult::NoIssuer)
    );

    //
    // Path Payment Strict Receive Result
    //

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAACAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAAAAyzXIcEd0vK9XlVfmjyQE9QpJjOLzYUN5orR0N+Dz+QAAAABVVNEAAAAAAAqg0ayXzXGPwPxfJ6TMpldG5JTYoiaEeRCPhBaGuhjrgAAAAAAAw1AAAAAAA==",
        InnerOperationResult::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::Success(_))
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC////9wAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAA==",
        InnerOperationResult::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::NoIssuer(_))
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC/////wAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_underfunded,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC/////gAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::Underfunded)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_source_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC/////QAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(
            PathPaymentStrictReceiveResult::SourceNoTrust
        )
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_source_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC/////AAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(
            PathPaymentStrictReceiveResult::SourceNotAuthorized
        )
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_no_destination,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC////+wAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(
            PathPaymentStrictReceiveResult::NoDestination
        )
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC////+gAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::NoTrust)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC////+QAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(
            PathPaymentStrictReceiveResult::NotAuthorized
        )
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_line_full,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC////+AAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::LineFull)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_too_few_offers,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC////9gAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(
            PathPaymentStrictReceiveResult::TooFewOffers
        )
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_offer_cross_self,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC////9QAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(
            PathPaymentStrictReceiveResult::OfferCrossSelf
        )
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_receive_over_send_max,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAC////9AAAAAA=",
        InnerOperationResult::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::OverSendMax)
    );

    //
    // Path Payment Strict Receive Result
    //

    impl_inner_op_result_test!(
        test_path_payment_strict_send_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAANAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAAAAyzXIcEd0vK9XlVfmjyQE9QpJjOLzYUN5orR0N+Dz+QAAAABVVNEAAAAAAAqg0ayXzXGPwPxfJ6TMpldG5JTYoiaEeRCPhBaGuhjrgAAAAAAAw1AAAAAAA==",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::Success(_))
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN////9wAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAA==",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::NoIssuer(_))
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN/////wAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_underfunded,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN/////gAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::Underfunded)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_source_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN/////QAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::SourceNoTrust)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_source_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN/////AAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(
            PathPaymentStrictSendResult::SourceNotAuthorized
        )
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_no_destination,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN////+wAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::NoDestination)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN////+gAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::NoTrust)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN////+QAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::NotAuthorized)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_line_full,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN////+AAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::LineFull)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_too_few_offers,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN////9gAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::TooFewOffers)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_offer_cross_self,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN////9QAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(PathPaymentStrictSendResult::OfferCrossSelf)
    );

    impl_inner_op_result_test!(
        test_path_payment_strict_send_under_destination_min,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAN////9AAAAAA=",
        InnerOperationResult::PathPaymentStrictSend(
            PathPaymentStrictSendResult::UnderDestinationMin
        )
    );

    //
    // Manage Sell Offer Result
    //

    impl_inner_op_result_test!(
        test_manage_sell_offer_success_created,
        "AAAAAACYloD/////AAAAAQAAAAAAAAADAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAAAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAAABNIAAAAAAAAAAVVTRAAAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAJiWgAAAA+gAABEYAAAAAQAAAAAAAAAA",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Created(_), .. })
        )
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_success_updated,
        "AAAAAACYloD/////AAAAAQAAAAAAAAADAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAABAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAAABNIAAAAAAAAAAVVTRAAAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAJiWgAAAA+gAABEYAAAAAQAAAAAAAAAA",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Updated(_), .. })
        )
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_success_deleted,
        "AAAAAACYloD/////AAAAAQAAAAAAAAADAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAACAAAAAA==",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Deleted, .. })
        )
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD/////wAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_sell_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD/////gAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::SellNoTrust)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_buy_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD/////QAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::BuyNoTrust)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_sell_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD/////AAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::SellNotAuthorized)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_buy_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD////+wAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::BuyNotAuthorized)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_line_full,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD////+gAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::LineFull)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_underfunded,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD////+QAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::Underfunded)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_cross_self,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD////+AAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::CrossSelf)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_sell_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD////9wAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::SellNoIssuer)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_buy_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD////9gAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::BuyNoIssuer)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_not_found,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD////9QAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::NotFound)
    );

    impl_inner_op_result_test!(
        test_manage_sell_offer_low_reserve,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAD////9AAAAAA=",
        InnerOperationResult::ManageSellOffer(ManageSellOfferResult::LowReserve)
    );

    //
    // Manage Buy Offer Result
    //

    impl_inner_op_result_test!(
        test_manage_buy_offer_success_created,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAMAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAAAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAAABNIAAAAAAAAAAVVTRAAAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAJiWgAAAA+gAABEYAAAAAQAAAAAAAAAA",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Created(_), .. })
        )
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_success_updated,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAMAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAABAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAAABNIAAAAAAAAAAVVTRAAAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAJiWgAAAA+gAABEYAAAAAQAAAAAAAAAA",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Updated(_), .. })
        )
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_success_deleted,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAMAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAACAAAAAA==",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Deleted, .. })
        )
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM/////wAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_sell_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM/////gAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::SellNoTrust)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_buy_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM/////QAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::BuyNoTrust)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_sell_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM/////AAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::SellNotAuthorized)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_buy_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM////+wAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::BuyNotAuthorized)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_line_full,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM////+gAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::LineFull)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_underfunded,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM////+QAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::Underfunded)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_cross_self,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM////+AAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::CrossSelf)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_sell_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM////9wAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::SellNoIssuer)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_buy_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM////9gAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::BuyNoIssuer)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_not_found,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM////9QAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::NotFound)
    );

    impl_inner_op_result_test!(
        test_manage_buy_offer_low_reserve,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAM////9AAAAAA=",
        InnerOperationResult::ManageBuyOffer(ManageBuyOfferResult::LowReserve)
    );

    //
    // Create Passive Sell Offer Result
    //
    //"AAAAAA AAA+j /////AAAAAQAAAAAAAAAE/////wAAAAA=",
    //    "AAAAAA CYloD /////AAAAAQAAAAAAAAAD/////wAAAAA=",

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_success_created,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAEAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAAAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAAABNIAAAAAAAAAAVVTRAAAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAJiWgAAAA+gAABEYAAAAAQAAAAAAAAAA",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Created(_), .. })
        )
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_success_updated,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAEAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAABAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAAABNIAAAAAAAAAAVVTRAAAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAJiWgAAAA+gAABEYAAAAAQAAAAAAAAAA",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Updated(_), .. })
        )
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_success_deleted,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAEAAAAAAAAAAEAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAAAE0gAAAAAAAAAAAJiWgAAAAAFVU0QAAAAAACqDRrJfNcY/A/F8npMymV0bklNiiJoR5EI+EFoa6GOuAAAAAAADDUAAAAACAAAAAA==",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::Success(
            ManageOfferResultSuccess { offer: OfferResult::Deleted, .. })
        )
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE/////wAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_sell_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE/////gAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::SellNoTrust)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_buy_no_trust,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE/////QAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::BuyNoTrust)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_sell_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE/////AAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::SellNotAuthorized)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_buy_not_authorized,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE////+wAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::BuyNotAuthorized)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_line_full,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE////+gAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::LineFull)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_underfunded,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE////+QAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::Underfunded)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_cross_self,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE////+AAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::CrossSelf)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_sell_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE////9wAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::SellNoIssuer)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_buy_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE////9gAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::BuyNoIssuer)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_not_found,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE////9QAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::NotFound)
    );

    impl_inner_op_result_test!(
        test_create_passive_sell_offer_low_reserve,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAE////9AAAAAA=",
        InnerOperationResult::CreatePassiveSellOffer(ManageSellOfferResult::LowReserve)
    );

    //
    // Set Options Result
    //

    impl_inner_op_result_test!(
        test_set_options_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAFAAAAAAAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::Success)
    );

    impl_inner_op_result_test!(
        test_set_options_low_reserve,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF/////wAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::LowReserve)
    );

    impl_inner_op_result_test!(
        test_set_options_too_many_signers,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF/////gAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::TooManySigners)
    );

    impl_inner_op_result_test!(
        test_set_options_bad_flags,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF/////QAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::BadFlags)
    );

    impl_inner_op_result_test!(
        test_set_options_invalid_inflation,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF/////AAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::InvalidInflation)
    );

    impl_inner_op_result_test!(
        test_set_options_cant_change,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF////+wAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::CantChange)
    );

    impl_inner_op_result_test!(
        test_set_options_unknown_flag,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF////+gAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::UnknownFlag)
    );

    impl_inner_op_result_test!(
        test_set_options_threshold_out_of_range,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF////+QAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::ThresholdOutOfRange)
    );

    impl_inner_op_result_test!(
        test_set_options_bad_signer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF////+AAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::BadSigner)
    );

    impl_inner_op_result_test!(
        test_set_options_invalid_home_domain,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAF////9wAAAAA=",
        InnerOperationResult::SetOptions(SetOptionsResult::InvalidHomeDomain)
    );

    //
    // Change Trust Result
    //

    impl_inner_op_result_test!(
        test_change_trust_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAGAAAAAAAAAAA=",
        InnerOperationResult::ChangeTrust(ChangeTrustResult::Success)
    );

    impl_inner_op_result_test!(
        test_change_trust_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAG/////wAAAAA=",
        InnerOperationResult::ChangeTrust(ChangeTrustResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_change_trust_no_issuer,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAG/////gAAAAA=",
        InnerOperationResult::ChangeTrust(ChangeTrustResult::NoIssuer)
    );

    impl_inner_op_result_test!(
        test_change_trust_invalid_limit,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAG/////QAAAAA=",
        InnerOperationResult::ChangeTrust(ChangeTrustResult::InvalidLimit)
    );

    impl_inner_op_result_test!(
        test_change_trust_low_reserve,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAG/////AAAAAA=",
        InnerOperationResult::ChangeTrust(ChangeTrustResult::LowReserve)
    );

    impl_inner_op_result_test!(
        test_change_trust_self_not_allowed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAG////+wAAAAA=",
        InnerOperationResult::ChangeTrust(ChangeTrustResult::SelfNotAllowed)
    );

    //
    // Allow Trust Result
    //

    impl_inner_op_result_test!(
        test_allow_trust_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAHAAAAAAAAAAA=",
        InnerOperationResult::AllowTrust(AllowTrustResult::Success)
    );

    impl_inner_op_result_test!(
        test_allow_trust_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAH/////wAAAAA=",
        InnerOperationResult::AllowTrust(AllowTrustResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_allow_trust_no_trustline,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAH/////gAAAAA=",
        InnerOperationResult::AllowTrust(AllowTrustResult::NoTrustLine)
    );

    impl_inner_op_result_test!(
        test_allow_trust_trust_not_required,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAH/////QAAAAA=",
        InnerOperationResult::AllowTrust(AllowTrustResult::TrustNotRequired)
    );

    impl_inner_op_result_test!(
        test_allow_trust_cant_revoke,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAH/////AAAAAA=",
        InnerOperationResult::AllowTrust(AllowTrustResult::CantRevoke)
    );

    impl_inner_op_result_test!(
        test_allow_trust_self_not_allowed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAH////+wAAAAA=",
        InnerOperationResult::AllowTrust(AllowTrustResult::SelfNotAllowed)
    );

    //
    // Account Merge Result
    //

    impl_inner_op_result_test!(
        test_accout_merge_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAIAAAAAAAAAAAF9eEAAAAAAA==",
        InnerOperationResult::AccountMerge(AccountMergeResult::Success(_))
    );

    impl_inner_op_result_test!(
        test_accout_merge_malformed,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAI/////wAAAAA=",
        InnerOperationResult::AccountMerge(AccountMergeResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_accout_merge_no_account,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAI/////gAAAAA=",
        InnerOperationResult::AccountMerge(AccountMergeResult::NoAccount)
    );

    impl_inner_op_result_test!(
        test_accout_merge_immutable_set,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAI/////QAAAAA=",
        InnerOperationResult::AccountMerge(AccountMergeResult::ImmutableSet)
    );

    impl_inner_op_result_test!(
        test_accout_merge_has_sub_entries,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAI/////AAAAAA=",
        InnerOperationResult::AccountMerge(AccountMergeResult::HasSubEntries)
    );

    impl_inner_op_result_test!(
        test_accout_merge_sequence_too_far,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAI////+wAAAAA=",
        InnerOperationResult::AccountMerge(AccountMergeResult::SequenceTooFar)
    );

    impl_inner_op_result_test!(
        test_accout_merge_destination_full,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAI////+gAAAAA=",
        InnerOperationResult::AccountMerge(AccountMergeResult::DestinationFull)
    );

    impl_inner_op_result_test!(
        test_accout_merge_is_sponsor,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAI////+QAAAAA=",
        InnerOperationResult::AccountMerge(AccountMergeResult::IsSponsor)
    );

    //
    // Inflation Result
    //

    impl_inner_op_result_test!(
        test_inflation_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAJAAAAAAAAAAIAAAAAKoNGsl81xj8D8XyekzKZXRuSU2KImhHkQj4QWhroY64AAAAAAJiWgAAAAAADLNchwR3S8r1eVV+aPJAT1CkmM4vNhQ3mitHQ34PP5AAAAAABMS0AAAAAAA==",
        InnerOperationResult::Inflation(InflationResult::Success(_))
    );

    impl_inner_op_result_test!(
        test_inflation_no_time,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAJ/////wAAAAA=",
        InnerOperationResult::Inflation(InflationResult::NotTime)
    );

    //
    // Manage Data Result
    //

    impl_inner_op_result_test!(
        test_manage_data_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAKAAAAAAAAAAA=",
        InnerOperationResult::ManageData(ManageDataResult::Success)
    );

    impl_inner_op_result_test!(
        test_manage_data_not_supported_yet,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAK/////wAAAAA=",
        InnerOperationResult::ManageData(ManageDataResult::NotSupportedYet)
    );

    impl_inner_op_result_test!(
        test_manage_data_name_not_found,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAK/////gAAAAA=",
        InnerOperationResult::ManageData(ManageDataResult::NameNotFound)
    );

    impl_inner_op_result_test!(
        test_manage_data_low_reserve,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAK/////QAAAAA=",
        InnerOperationResult::ManageData(ManageDataResult::LowReserve)
    );

    impl_inner_op_result_test!(
        test_manage_data_invalid_name,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAK/////AAAAAA=",
        InnerOperationResult::ManageData(ManageDataResult::InvalidName)
    );

    //
    // Bump Sequence Result
    //

    impl_inner_op_result_test!(
        test_bump_sequence_success,
        "AAAAAACYloD/////AAAAAQAAAAAAAAALAAAAAAAAAAA=",
        InnerOperationResult::BumpSequence(BumpSequenceResult::Success)
    );

    impl_inner_op_result_test!(
        test_bump_sequence_bad_sequence,
        "AAAAAACYloD/////AAAAAQAAAAAAAAAL/////wAAAAA=",
        InnerOperationResult::BumpSequence(BumpSequenceResult::BadSeq)
    );

    //
    // Create Claimable Balance Result
    //

    impl_inner_op_result_test!(
        test_create_claimable_balance_success,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAOAAAAAAAAAAAHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwAAAAA=",
        InnerOperationResult::CreateClaimableBalance(CreateClaimableBalanceResult::Success(_))
    );

    impl_inner_op_result_test!(
        test_create_claimable_balance_malformed,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAO/////wAAAAA=",
        InnerOperationResult::CreateClaimableBalance(CreateClaimableBalanceResult::Malformed)
    );

    impl_inner_op_result_test!(
        test_create_claimable_balance_low_reserve,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAO/////gAAAAA=",
        InnerOperationResult::CreateClaimableBalance(CreateClaimableBalanceResult::LowReserve)
    );

    impl_inner_op_result_test!(
        test_create_claimable_balance_no_trust,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAO/////QAAAAA=",
        InnerOperationResult::CreateClaimableBalance(CreateClaimableBalanceResult::NoTrust)
    );

    impl_inner_op_result_test!(
        test_create_claimable_balance_not_authorized,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAO/////AAAAAA=",
        InnerOperationResult::CreateClaimableBalance(CreateClaimableBalanceResult::NotAuthorized)
    );

    impl_inner_op_result_test!(
        test_create_claimable_balance_underfunded,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAO////+wAAAAA=",
        InnerOperationResult::CreateClaimableBalance(CreateClaimableBalanceResult::Underfunded)
    );

    //
    // Begin Sponsoring Future Reserves Result
    //

    impl_inner_op_result_test!(
        test_begin_sponsoring_future_reserves_success,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAQAAAAAAAAAAA=",
        InnerOperationResult::BeginSponsoringFutureReserves(
            BeginSponsoringFutureReservesResult::Success
        )
    );

    impl_inner_op_result_test!(
        test_begin_sponsoring_future_reserves_malformed,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAQ/////wAAAAA=",
        InnerOperationResult::BeginSponsoringFutureReserves(
            BeginSponsoringFutureReservesResult::Malformed
        )
    );

    impl_inner_op_result_test!(
        test_begin_sponsoring_future_reserves_already_sponsored,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAQ/////gAAAAA=",
        InnerOperationResult::BeginSponsoringFutureReserves(
            BeginSponsoringFutureReservesResult::AlreadySponsored
        )
    );

    impl_inner_op_result_test!(
        test_begin_sponsoring_future_reserves_recursive,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAQ/////QAAAAA=",
        InnerOperationResult::BeginSponsoringFutureReserves(
            BeginSponsoringFutureReservesResult::Recursive
        )
    );

    //
    // End Sponsoring Future Reserves Result
    //

    impl_inner_op_result_test!(
        test_end_sponsoring_future_reserves_success,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAARAAAAAAAAAAA=",
        InnerOperationResult::EndSponsoringFutureReserves(
            EndSponsoringFutureReservesResult::Success
        )
    );

    impl_inner_op_result_test!(
        test_end_sponsoring_future_reserves_not_sponsored,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAR/////wAAAAA=",
        InnerOperationResult::EndSponsoringFutureReserves(
            EndSponsoringFutureReservesResult::NotSponsored
        )
    );

    //
    // Revoke Sponsorship Result
    //

    impl_inner_op_result_test!(
        test_revoke_sponsorship_success,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAASAAAAAAAAAAA=",
        InnerOperationResult::RevokeSponsorship(RevokeSponsorshipResult::Success)
    );

    impl_inner_op_result_test!(
        test_revoke_sponsorship_does_not_exist,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAS/////wAAAAA=",
        InnerOperationResult::RevokeSponsorship(RevokeSponsorshipResult::DoesNotExist)
    );

    impl_inner_op_result_test!(
        test_revoke_sponsorship_not_sponsor,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAS/////gAAAAA=",
        InnerOperationResult::RevokeSponsorship(RevokeSponsorshipResult::NotSponsor)
    );

    impl_inner_op_result_test!(
        test_revoke_sponsorship_low_reserve,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAS/////QAAAAA=",
        InnerOperationResult::RevokeSponsorship(RevokeSponsorshipResult::LowReserve)
    );

    impl_inner_op_result_test!(
        test_revoke_sponsorship_only_transferable,
        "AAAAAAAAA+j/////AAAAAQAAAAAAAAAS/////AAAAAA=",
        InnerOperationResult::RevokeSponsorship(RevokeSponsorshipResult::OnlyTransferable)
    );
}
