//! Operations that mutate the ledger state.
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

mod account_merge;
mod allow_trust;
mod begin_sponsoring_future_reserves;
mod bump_sequence;
mod change_trust;
mod claim_claimable_balance;
mod create_account;
mod create_claimable_balance;
mod create_passive_sell_offer;
mod end_sponsoring_future_reserves;
mod inflation;
mod manage_buy_offer;
mod manage_data;
mod manage_sell_offer;
mod path_payment_strict_receive;
mod path_payment_strict_send;
mod payment;
mod revoke_sponsorship;
mod set_options;

pub use account_merge::{AccountMergeOperation, AccountMergeOperationBuilder};
pub use allow_trust::{AllowTrustOperation, AllowTrustOperationBuilder};
pub use begin_sponsoring_future_reserves::{
    BeginSponsoringFutureReservesOperation, BeginSponsoringFutureReservesOperationBuilder,
};
pub use bump_sequence::{BumpSequenceOperation, BumpSequenceOperationBuilder};
pub use change_trust::{ChangeTrustOperation, ChangeTrustOperationBuilder};
pub use claim_claimable_balance::{
    ClaimClaimableBalanceOperation, ClaimClaimableBalanceOperationBuilder,
};
pub use create_account::{CreateAccountOperation, CreateAccountOperationBuilder};
pub use create_claimable_balance::{
    CreateClaimableBalanceOperation, CreateClaimableBalanceOperationBuilder,
};
pub use create_passive_sell_offer::{
    CreatePassiveSellOfferOperation, CreatePassiveSellOfferOperationBuilder,
};
pub use end_sponsoring_future_reserves::{
    EndSponsoringFutureReservesOperation, EndSponsoringFutureReservesOperationBuilder,
};
pub use inflation::{InflationOperation, InflationOperationBuilder};
pub use manage_buy_offer::{ManageBuyOfferOperation, ManageBuyOfferOperationBuilder};
pub use manage_data::{ManageDataOperation, ManageDataOperationBuilder};
pub use manage_sell_offer::{ManageSellOfferOperation, ManageSellOfferOperationBuilder};
pub use path_payment_strict_receive::{
    PathPaymentStrictReceiveOperation, PathPaymentStrictReceiveOperationBuilder,
};
pub use path_payment_strict_send::{
    PathPaymentStrictSendOperation, PathPaymentStrictSendOperationBuilder,
};
pub use payment::{PaymentOperation, PaymentOperationBuilder};
pub use revoke_sponsorship::{RevokeSponsorshipOperation, RevokeSponsorshipOperationBuilder};
pub use set_options::{SetOptionsOperation, SetOptionsOperationBuilder};

/// Operations on a Stellar network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// Create and fund a non existing account.
    CreateAccount(CreateAccountOperation),
    /// Create a payment.
    Payment(PaymentOperation),
    /// Send the specified amount of asset, optionally through a path.
    PathPaymentStrictReceive(PathPaymentStrictReceiveOperation),
    /// Create, update, or delete a sell offer.
    ManageSellOffer(ManageSellOfferOperation),
    /// Create a passive sell offer. This offer won't consume a counter offer that exactly matches this offer.
    CreatePassiveSellOffer(CreatePassiveSellOfferOperation),
    /// Change the account options.
    SetOptions(SetOptionsOperation),
    /// Add, remove, or update a trust line for the given asset.
    ChangeTrust(ChangeTrustOperation),
    /// Authorize another account to hold your account's credit aset.
    AllowTrust(AllowTrustOperation),
    /// Transfer all of account balance into destination account.
    AccountMerge(AccountMergeOperation),
    /// Generate the inflation.
    Inflation(InflationOperation),
    /// Add o remove data entry.
    ManageData(ManageDataOperation),
    /// Bumps the account sequence number.
    BumpSequence(BumpSequenceOperation),
    /// Create, update, or delete a buy offer.
    ManageBuyOffer(ManageBuyOfferOperation),
    /// Send the specified amount of asset, optionally through a path.
    PathPaymentStrictSend(PathPaymentStrictSendOperation),
    /// Create a new claimable balance.
    CreateClaimableBalance(CreateClaimableBalanceOperation),
    /// Claim a claimable balance.
    ClaimClaimableBalance(ClaimClaimableBalanceOperation),
    /// Begin sponsoring future reserves for an account.
    /// Needs a matching EndSponsoringFutureReserves in the same transaction.
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOperation),
    /// Ends a BeginSponsoringFutureReserves operation in the transaction.
    EndSponsoringFutureReserves(EndSponsoringFutureReservesOperation),
    /// Revoke a reserve sponsorship.
    RevokeSponsorship(RevokeSponsorshipOperation),
}

impl Operation {
    /// Creates a new create account operation builder.
    pub fn new_create_account() -> CreateAccountOperationBuilder {
        CreateAccountOperationBuilder::new()
    }

    /// Creates a new payment operation builder.
    pub fn new_payment() -> PaymentOperationBuilder {
        PaymentOperationBuilder::new()
    }

    /// Creates a new path payment strict receive operation builder.
    pub fn new_path_payment_strict_receive() -> PathPaymentStrictReceiveOperationBuilder {
        PathPaymentStrictReceiveOperationBuilder::new()
    }

    /// Creates a new manage sell offer operation builder.
    pub fn new_manage_sell_offer() -> ManageSellOfferOperationBuilder {
        ManageSellOfferOperationBuilder::new()
    }

    /// Creates a new create passive sell offer operation builder.
    pub fn new_create_passive_sell_offer() -> CreatePassiveSellOfferOperationBuilder {
        CreatePassiveSellOfferOperationBuilder::new()
    }

    /// Creates a new set options operation builder.
    pub fn new_set_options() -> SetOptionsOperationBuilder {
        SetOptionsOperationBuilder::new()
    }

    /// Creates a new change trust operation builder.
    pub fn new_change_trust() -> ChangeTrustOperationBuilder {
        ChangeTrustOperationBuilder::new()
    }

    /// Creates a new allow trust operation builder.
    pub fn new_allow_trust() -> AllowTrustOperationBuilder {
        AllowTrustOperationBuilder::new()
    }

    /// Creates a new account merge operation builder.
    pub fn new_account_merge() -> AccountMergeOperationBuilder {
        AccountMergeOperationBuilder::new()
    }

    /// Creates a new inflation operation builder.
    pub fn new_inflation() -> InflationOperationBuilder {
        InflationOperationBuilder::new()
    }

    /// Creates a new manage data operation builder.
    pub fn new_manage_data() -> ManageDataOperationBuilder {
        ManageDataOperationBuilder::new()
    }

    /// Creates a new bump sequence operation builder.
    pub fn new_bump_sequence() -> BumpSequenceOperationBuilder {
        BumpSequenceOperationBuilder::new()
    }

    /// Creates a new manage buy offer operation builder.
    pub fn new_manage_buy_offer() -> ManageBuyOfferOperationBuilder {
        ManageBuyOfferOperationBuilder::new()
    }

    /// Creates a new path payment strict send operation builder.
    pub fn new_path_payment_strict_send() -> PathPaymentStrictSendOperationBuilder {
        PathPaymentStrictSendOperationBuilder::new()
    }

    /// Creates a new create claimable balance operation builder.
    pub fn new_create_claimable_balance() -> CreateClaimableBalanceOperationBuilder {
        CreateClaimableBalanceOperationBuilder::new()
    }

    /// Creates a new claim claimable balance operation builder.
    pub fn new_claim_claimable_balance() -> ClaimClaimableBalanceOperationBuilder {
        ClaimClaimableBalanceOperationBuilder::new()
    }

    /// Creates a new begin sponsoring future reserves operation builder.
    pub fn new_begin_sponsoring_future_reserves() -> BeginSponsoringFutureReservesOperationBuilder {
        BeginSponsoringFutureReservesOperationBuilder::new()
    }

    /// Creates a new end sponsoring future reserves operation builder.
    pub fn new_end_sponsoring_future_reserves() -> EndSponsoringFutureReservesOperationBuilder {
        EndSponsoringFutureReservesOperationBuilder::new()
    }

    /// Creates a new revoke sponsorship operation builder.
    pub fn new_revoke_sponsorship() -> RevokeSponsorshipOperationBuilder {
        RevokeSponsorshipOperationBuilder::new()
    }

    /// If the operation is a CreateAccount, returns its value. Returns None otherwise.
    pub fn as_create_account(&self) -> Option<&CreateAccountOperation> {
        match *self {
            Operation::CreateAccount(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a CreateAccount, returns its mutable value. Returns None otherwise.
    pub fn as_create_account_mut(&mut self) -> Option<&mut CreateAccountOperation> {
        match *self {
            Operation::CreateAccount(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a CreateAccount.
    pub fn is_create_account(&self) -> bool {
        self.as_create_account().is_some()
    }

    /// If the operation is a Payment, returns its value. Returns None otherwise.
    pub fn as_payment(&self) -> Option<&PaymentOperation> {
        match *self {
            Operation::Payment(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a Payment, returns its value. Returns None otherwise.
    pub fn as_payment_mut(&mut self) -> Option<&mut PaymentOperation> {
        match *self {
            Operation::Payment(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a Payment.
    pub fn is_payment(&self) -> bool {
        self.as_payment().is_some()
    }

    /// If the operation is a PathPaymentStrictReceive, returns its value. Returns None otherwise.
    pub fn as_path_payment_strict_receive(&self) -> Option<&PathPaymentStrictReceiveOperation> {
        match *self {
            Operation::PathPaymentStrictReceive(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a PathPaymentStrictReceive, returns its mutable value. Returns None otherwise.
    pub fn as_path_payment_strict_receive_mut(
        &mut self,
    ) -> Option<&mut PathPaymentStrictReceiveOperation> {
        match *self {
            Operation::PathPaymentStrictReceive(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a PathPaymentStrictReceive.
    pub fn is_path_payment_strict_receive(&self) -> bool {
        self.as_path_payment_strict_receive().is_some()
    }

    /// If the operation is a ManageSellOffer, returns its value. Returns None otherwise.
    pub fn as_manage_sell_offer(&self) -> Option<&ManageSellOfferOperation> {
        match *self {
            Operation::ManageSellOffer(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a ManageSellOffer, returns its mutable value. Returns None otherwise.
    pub fn as_manage_sell_offer_mut(&mut self) -> Option<&mut ManageSellOfferOperation> {
        match *self {
            Operation::ManageSellOffer(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a ManageSellOffer.
    pub fn is_manage_sell_offer(&self) -> bool {
        self.as_manage_sell_offer().is_some()
    }

    /// If the operation is a CreatePassiveSellOffer, returns its value. Returns None otherwise.
    pub fn as_create_passive_sell_offer(&self) -> Option<&CreatePassiveSellOfferOperation> {
        match *self {
            Operation::CreatePassiveSellOffer(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a CreatePassiveSellOffer, returns its mutable value. Returns None otherwise.
    pub fn as_create_passive_sell_offer_mut(
        &mut self,
    ) -> Option<&mut CreatePassiveSellOfferOperation> {
        match *self {
            Operation::CreatePassiveSellOffer(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a CreatePassiveSellOffer.
    pub fn is_create_passive_sell_offer(&self) -> bool {
        self.as_create_passive_sell_offer().is_some()
    }

    /// If the operation is a SetOptions, returns its value. Returns None otherwise.
    pub fn as_set_options(&self) -> Option<&SetOptionsOperation> {
        match *self {
            Operation::SetOptions(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a SetOptions, returns its mutable value. Returns None otherwise.
    pub fn as_set_options_mut(&mut self) -> Option<&mut SetOptionsOperation> {
        match *self {
            Operation::SetOptions(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a SetOptions.
    pub fn is_set_options(&self) -> bool {
        self.as_set_options().is_some()
    }

    /// If the operation is a ChangeTrust, returns its value. Returns None otherwise.
    pub fn as_change_trust(&self) -> Option<&ChangeTrustOperation> {
        match *self {
            Operation::ChangeTrust(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a ChangeTrust, returns its mutable value. Returns None otherwise.
    pub fn as_change_trust_mut(&mut self) -> Option<&ChangeTrustOperation> {
        match *self {
            Operation::ChangeTrust(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a ChangeTrust.
    pub fn is_change_trust(&self) -> bool {
        self.as_change_trust().is_some()
    }

    /// If the operation is a AllowTrust, returns its value. Returns None otherwise.
    pub fn as_allow_trust(&self) -> Option<&AllowTrustOperation> {
        match *self {
            Operation::AllowTrust(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a AllowTrust, returns its mutable value. Returns None otherwise.
    pub fn as_allow_trust_mut(&mut self) -> Option<&mut AllowTrustOperation> {
        match *self {
            Operation::AllowTrust(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a AllowTrust.
    pub fn is_allow_trust(&self) -> bool {
        self.as_allow_trust().is_some()
    }

    /// If the operation is a AccountMerge, returns its value. Returns None otherwise.
    pub fn as_account_merge(&self) -> Option<&AccountMergeOperation> {
        match *self {
            Operation::AccountMerge(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a AccountMerge, returns its mutable value. Returns None otherwise.
    pub fn as_account_merge_mut(&mut self) -> Option<&mut AccountMergeOperation> {
        match *self {
            Operation::AccountMerge(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a AccountMerge.
    pub fn is_account_merge(&self) -> bool {
        self.as_account_merge().is_some()
    }

    /// If the operation is a Inflation, returns its value. Returns None otherwise.
    pub fn as_inflation(&self) -> Option<&InflationOperation> {
        match *self {
            Operation::Inflation(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a Inflation, returns its mutable value. Returns None otherwise.
    pub fn as_inflation_mut(&mut self) -> Option<&mut InflationOperation> {
        match *self {
            Operation::Inflation(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a Inflation.
    pub fn is_inflation(&self) -> bool {
        self.as_inflation().is_some()
    }

    /// If the operation is a ManageData, returns its value. Returns None otherwise.
    pub fn as_manage_data(&self) -> Option<&ManageDataOperation> {
        match *self {
            Operation::ManageData(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a ManageData, returns its mutable value. Returns None otherwise.
    pub fn as_manage_data_mut(&mut self) -> Option<&mut ManageDataOperation> {
        match *self {
            Operation::ManageData(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a ManageData.
    pub fn is_manage_data(&self) -> bool {
        self.as_manage_data().is_some()
    }

    /// If the operation is a BumpSequence, returns its value. Returns None otherwise.
    pub fn as_bump_sequence(&self) -> Option<&BumpSequenceOperation> {
        match *self {
            Operation::BumpSequence(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a BumpSequence, returns its mutable value. Returns None otherwise.
    pub fn as_bump_sequence_mut(&mut self) -> Option<&mut BumpSequenceOperation> {
        match *self {
            Operation::BumpSequence(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a BumpSequence.
    pub fn is_bump_sequence(&self) -> bool {
        self.as_bump_sequence().is_some()
    }

    /// If the operation is a ManageBuyOffer, returns its value. Returns None otherwise.
    pub fn as_manage_buy_offer(&self) -> Option<&ManageBuyOfferOperation> {
        match *self {
            Operation::ManageBuyOffer(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a ManageBuyOffer, returns its mutable value. Returns None otherwise.
    pub fn as_manage_buy_offer_mut(&mut self) -> Option<&mut ManageBuyOfferOperation> {
        match *self {
            Operation::ManageBuyOffer(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a ManageBuyOffer.
    pub fn is_manage_buy_offer(&self) -> bool {
        self.as_manage_buy_offer().is_some()
    }

    /// If the operation is a PathPaymentStrictSend, returns its value. Returns None otherwise.
    pub fn as_path_payment_strict_send(&self) -> Option<&PathPaymentStrictSendOperation> {
        match *self {
            Operation::PathPaymentStrictSend(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a PathPaymentStrictSend, returns its mutable value. Returns None otherwise.
    pub fn as_path_payment_strict_send_mut(
        &mut self,
    ) -> Option<&mut PathPaymentStrictSendOperation> {
        match *self {
            Operation::PathPaymentStrictSend(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a PathPaymentStrictSend.
    pub fn is_path_payment_strict_send(&self) -> bool {
        self.as_path_payment_strict_send().is_some()
    }

    /// If the operation is a CreateClaimableBalance, returns its value. Returns None otherwise.
    pub fn as_create_claimable_balance(&self) -> Option<&CreateClaimableBalanceOperation> {
        match *self {
            Operation::CreateClaimableBalance(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a CreateClaimableBalance, returns its mutable value. Returns None otherwise.
    pub fn as_create_claimable_balance_mut(
        &mut self,
    ) -> Option<&mut CreateClaimableBalanceOperation> {
        match *self {
            Operation::CreateClaimableBalance(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a CreateClaimableBalance.
    pub fn is_create_claimable_balance(&self) -> bool {
        self.as_create_claimable_balance().is_some()
    }

    /// If the operation is a ClaimClaimableBalance, returns its value. Returns None otherwise.
    pub fn as_claim_claimable_balance(&self) -> Option<&ClaimClaimableBalanceOperation> {
        match *self {
            Operation::ClaimClaimableBalance(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a ClaimClaimableBalance, returns its mutable value. Returns None otherwise.
    pub fn as_claim_claimable_balance_mut(
        &mut self,
    ) -> Option<&mut ClaimClaimableBalanceOperation> {
        match *self {
            Operation::ClaimClaimableBalance(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a ClaimClaimableBalance.
    pub fn is_claim_claimable_balance(&self) -> bool {
        self.as_claim_claimable_balance().is_some()
    }

    /// If the operation is a BeginSponsoringFutureReserves, returns its value. Returns None otherwise.
    pub fn as_begin_sponsoring_future_reserves(
        &self,
    ) -> Option<&BeginSponsoringFutureReservesOperation> {
        match *self {
            Operation::BeginSponsoringFutureReserves(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a BeginSponsoringFutureReserves, returns its mutable value. Returns None otherwise.
    pub fn as_begin_sponsoring_future_reserves_mut(
        &mut self,
    ) -> Option<&mut BeginSponsoringFutureReservesOperation> {
        match *self {
            Operation::BeginSponsoringFutureReserves(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a BeginSponsoringFutureReserves.
    pub fn is_begin_sponsoring_future_reserves(&self) -> bool {
        self.as_begin_sponsoring_future_reserves().is_some()
    }

    /// If the operation is a EndSponsoringFutureReserves, returns its value. Returns None otherwise.
    pub fn as_end_sponsoring_future_reserves(
        &self,
    ) -> Option<&EndSponsoringFutureReservesOperation> {
        match *self {
            Operation::EndSponsoringFutureReserves(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a EndSponsoringFutureReserves, returns its mutable value. Returns None otherwise.
    pub fn as_end_sponsoring_future_reserves_mut(
        &mut self,
    ) -> Option<&mut EndSponsoringFutureReservesOperation> {
        match *self {
            Operation::EndSponsoringFutureReserves(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a EndSponsoringFutureReserves.
    pub fn is_end_sponsoring_future_reserves(&self) -> bool {
        self.as_end_sponsoring_future_reserves().is_some()
    }

    /// If the operation is a RevokeSponsorship, returns its value. Returns None otherwise.
    pub fn as_revoke_sponsorship(&self) -> Option<&RevokeSponsorshipOperation> {
        match *self {
            Operation::RevokeSponsorship(ref op) => Some(op),
            _ => None,
        }
    }

    /// If the operation is a RevokeSponsorship, returns its mutable value. Returns None otherwise.
    pub fn as_revoke_sponsorship_mut(&mut self) -> Option<&mut RevokeSponsorshipOperation> {
        match *self {
            Operation::RevokeSponsorship(ref mut op) => Some(op),
            _ => None,
        }
    }

    /// Returns true if the operation is a RevokeSponsorship.
    pub fn is_revoke_sponsorship(&self) -> bool {
        self.as_revoke_sponsorship().is_some()
    }

    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        match self {
            Operation::CreateAccount(op) => op.source_account(),
            Operation::Payment(op) => op.source_account(),
            Operation::PathPaymentStrictReceive(op) => op.source_account(),
            Operation::ManageSellOffer(op) => op.source_account(),
            Operation::CreatePassiveSellOffer(op) => op.source_account(),
            Operation::SetOptions(op) => op.source_account(),
            Operation::ChangeTrust(op) => op.source_account(),
            Operation::AllowTrust(op) => op.source_account(),
            Operation::AccountMerge(op) => op.source_account(),
            Operation::Inflation(op) => op.source_account(),
            Operation::ManageData(op) => op.source_account(),
            Operation::BumpSequence(op) => op.source_account(),
            Operation::ManageBuyOffer(op) => op.source_account(),
            Operation::PathPaymentStrictSend(op) => op.source_account(),
            Operation::CreateClaimableBalance(op) => op.source_account(),
            Operation::ClaimClaimableBalance(op) => op.source_account(),
            Operation::BeginSponsoringFutureReserves(op) => op.source_account(),
            Operation::EndSponsoringFutureReserves(op) => op.source_account(),
            Operation::RevokeSponsorship(op) => op.source_account(),
        }
    }

    /// Retrieves a mutable reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        match self {
            Operation::CreateAccount(op) => op.source_account_mut(),
            Operation::Payment(op) => op.source_account_mut(),
            Operation::PathPaymentStrictReceive(op) => op.source_account_mut(),
            Operation::ManageSellOffer(op) => op.source_account_mut(),
            Operation::CreatePassiveSellOffer(op) => op.source_account_mut(),
            Operation::SetOptions(op) => op.source_account_mut(),
            Operation::ChangeTrust(op) => op.source_account_mut(),
            Operation::AllowTrust(op) => op.source_account_mut(),
            Operation::AccountMerge(op) => op.source_account_mut(),
            Operation::Inflation(op) => op.source_account_mut(),
            Operation::ManageData(op) => op.source_account_mut(),
            Operation::BumpSequence(op) => op.source_account_mut(),
            Operation::ManageBuyOffer(op) => op.source_account_mut(),
            Operation::PathPaymentStrictSend(op) => op.source_account_mut(),
            Operation::CreateClaimableBalance(op) => op.source_account_mut(),
            Operation::ClaimClaimableBalance(op) => op.source_account_mut(),
            Operation::BeginSponsoringFutureReserves(op) => op.source_account_mut(),
            Operation::EndSponsoringFutureReserves(op) => op.source_account_mut(),
            Operation::RevokeSponsorship(op) => op.source_account_mut(),
        }
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::Operation> {
        let source_account = match self.source_account() {
            None => None,
            Some(account) => Some(account.to_xdr()?),
        };
        let body = match self {
            Operation::CreateAccount(op) => op.to_xdr_operation_body()?,
            Operation::Payment(op) => op.to_xdr_operation_body()?,
            Operation::PathPaymentStrictReceive(op) => op.to_xdr_operation_body()?,
            Operation::ManageSellOffer(op) => op.to_xdr_operation_body()?,
            Operation::CreatePassiveSellOffer(op) => op.to_xdr_operation_body()?,
            Operation::SetOptions(op) => op.to_xdr_operation_body()?,
            Operation::ChangeTrust(op) => op.to_xdr_operation_body()?,
            Operation::AllowTrust(op) => op.to_xdr_operation_body()?,
            Operation::AccountMerge(op) => op.to_xdr_operation_body()?,
            Operation::Inflation(op) => op.to_xdr_operation_body()?,
            Operation::ManageData(op) => op.to_xdr_operation_body()?,
            Operation::BumpSequence(op) => op.to_xdr_operation_body()?,
            Operation::ManageBuyOffer(op) => op.to_xdr_operation_body()?,
            Operation::PathPaymentStrictSend(op) => op.to_xdr_operation_body()?,
            Operation::CreateClaimableBalance(op) => op.to_xdr_operation_body()?,
            Operation::ClaimClaimableBalance(op) => op.to_xdr_operation_body()?,
            Operation::BeginSponsoringFutureReserves(op) => op.to_xdr_operation_body()?,
            Operation::EndSponsoringFutureReserves(op) => op.to_xdr_operation_body()?,
            Operation::RevokeSponsorship(op) => op.to_xdr_operation_body()?,
        };
        Ok(xdr::Operation {
            source_account,
            body,
        })
    }

    /// Creates from the xdr object.
    pub fn from_xdr(x: &xdr::Operation) -> Result<Operation> {
        let source_account = match &x.source_account {
            None => None,
            Some(sa) => Some(MuxedAccount::from_xdr(&sa)?),
        };
        match &x.body {
            xdr::OperationBody::CreateAccount(op) => {
                let inner = CreateAccountOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::CreateAccount(inner))
            }
            xdr::OperationBody::Payment(op) => {
                let inner = PaymentOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::Payment(inner))
            }
            xdr::OperationBody::PathPaymentStrictReceive(op) => {
                let inner =
                    PathPaymentStrictReceiveOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::PathPaymentStrictReceive(inner))
            }
            xdr::OperationBody::ManageSellOffer(op) => {
                let inner = ManageSellOfferOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::ManageSellOffer(inner))
            }
            xdr::OperationBody::CreatePassiveSellOffer(op) => {
                let inner =
                    CreatePassiveSellOfferOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::CreatePassiveSellOffer(inner))
            }
            xdr::OperationBody::SetOptions(op) => {
                let inner = SetOptionsOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::SetOptions(inner))
            }
            xdr::OperationBody::ChangeTrust(op) => {
                let inner = ChangeTrustOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::ChangeTrust(inner))
            }
            xdr::OperationBody::AllowTrust(op) => {
                let inner = AllowTrustOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::AllowTrust(inner))
            }
            xdr::OperationBody::AccountMerge(op) => {
                let inner = AccountMergeOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::AccountMerge(inner))
            }
            xdr::OperationBody::Inflation(()) => {
                let inner = InflationOperation::from_xdr_operation_body(source_account)?;
                Ok(Operation::Inflation(inner))
            }
            xdr::OperationBody::ManageData(op) => {
                let inner = ManageDataOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::ManageData(inner))
            }
            xdr::OperationBody::BumpSequence(op) => {
                let inner = BumpSequenceOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::BumpSequence(inner))
            }
            xdr::OperationBody::ManageBuyOffer(op) => {
                let inner = ManageBuyOfferOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::ManageBuyOffer(inner))
            }
            xdr::OperationBody::PathPaymentStrictSend(op) => {
                let inner =
                    PathPaymentStrictSendOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::PathPaymentStrictSend(inner))
            }
            xdr::OperationBody::CreateClaimableBalance(op) => {
                let inner =
                    CreateClaimableBalanceOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::CreateClaimableBalance(inner))
            }
            xdr::OperationBody::ClaimClaimableBalance(op) => {
                let inner =
                    ClaimClaimableBalanceOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::ClaimClaimableBalance(inner))
            }
            xdr::OperationBody::BeginSponsoringFutureReserves(op) => {
                let inner = BeginSponsoringFutureReservesOperation::from_xdr_operation_body(
                    source_account,
                    op,
                )?;
                Ok(Operation::BeginSponsoringFutureReserves(inner))
            }
            xdr::OperationBody::EndSponsoringFutureReserves(()) => {
                let inner =
                    EndSponsoringFutureReservesOperation::from_xdr_operation_body(source_account)?;
                Ok(Operation::EndSponsoringFutureReserves(inner))
            }
            xdr::OperationBody::RevokeSponsorship(op) => {
                let inner =
                    RevokeSponsorshipOperation::from_xdr_operation_body(source_account, op)?;
                Ok(Operation::RevokeSponsorship(inner))
            }
        }
    }
}

impl XDRSerialize for Operation {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr_operation = self.to_xdr()?;
        xdr_operation.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for Operation {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_operation, bytes_read) =
            xdr::Operation::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = Operation::from_xdr(&xdr_operation)?;
        Ok((res, bytes_read))
    }
}
