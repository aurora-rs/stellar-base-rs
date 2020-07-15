use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

mod account_merge;
mod allow_trust;
mod bump_sequence;
mod change_trust;
mod create_account;
mod create_passive_sell_offer;
mod inflation;
mod manage_buy_offer;
mod manage_data;
mod manage_sell_offer;
mod path_payment_strict_receive;
mod path_payment_strict_send;
mod payment;
mod set_options;

pub use account_merge::{AccountMergeOperation, AccountMergeOperationBuilder};
pub use allow_trust::{AllowTrustOperation, AllowTrustOperationBuilder};
pub use bump_sequence::{BumpSequenceOperation, BumpSequenceOperationBuilder};
pub use change_trust::{ChangeTrustOperation, ChangeTrustOperationBuilder};
pub use create_account::{CreateAccountOperation, CreateAccountOperationBuilder};
pub use create_passive_sell_offer::{
    CreatePassiveSellOfferOperation, CreatePassiveSellOfferOperationBuilder,
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
pub use set_options::{SetOptionsOperation, SetOptionsOperationBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    CreateAccount(CreateAccountOperation),
    Payment(PaymentOperation),
    PathPaymentStrictReceive(PathPaymentStrictReceiveOperation),
    ManageSellOffer(ManageSellOfferOperation),
    CreatePassiveSellOffer(CreatePassiveSellOfferOperation),
    SetOptions(SetOptionsOperation),
    ChangeTrust(ChangeTrustOperation),
    AllowTrust(AllowTrustOperation),
    AccountMerge(AccountMergeOperation),
    Inflation(InflationOperation),
    ManageData(ManageDataOperation),
    BumpSequence(BumpSequenceOperation),
    ManageBuyOffer(ManageBuyOfferOperation),
    PathPaymentStrictSend(PathPaymentStrictSendOperation),
}

pub fn create_account() -> CreateAccountOperationBuilder {
    CreateAccountOperationBuilder::new()
}

pub fn payment() -> PaymentOperationBuilder {
    PaymentOperationBuilder::new()
}

pub fn path_payment_strict_receive() -> PathPaymentStrictReceiveOperationBuilder {
    PathPaymentStrictReceiveOperationBuilder::new()
}

pub fn manage_sell_offer() -> ManageSellOfferOperationBuilder {
    ManageSellOfferOperationBuilder::new()
}

pub fn create_passive_sell_offer() -> CreatePassiveSellOfferOperationBuilder {
    CreatePassiveSellOfferOperationBuilder::new()
}

pub fn set_options() -> SetOptionsOperationBuilder {
    SetOptionsOperationBuilder::new()
}

pub fn change_trust() -> ChangeTrustOperationBuilder {
    ChangeTrustOperationBuilder::new()
}

pub fn allow_trust() -> AllowTrustOperationBuilder {
    AllowTrustOperationBuilder::new()
}

pub fn account_merge() -> AccountMergeOperationBuilder {
    AccountMergeOperationBuilder::new()
}

pub fn inflation() -> InflationOperationBuilder {
    InflationOperationBuilder::new()
}

pub fn manage_data() -> ManageDataOperationBuilder {
    ManageDataOperationBuilder::new()
}

pub fn bump_sequence() -> BumpSequenceOperationBuilder {
    BumpSequenceOperationBuilder::new()
}

pub fn manage_buy_offer() -> ManageBuyOfferOperationBuilder {
    ManageBuyOfferOperationBuilder::new()
}

pub fn path_payment_strict_send() -> PathPaymentStrictSendOperationBuilder {
    PathPaymentStrictSendOperationBuilder::new()
}

impl Operation {
    pub fn create_account(&self) -> Option<&CreateAccountOperation> {
        match self {
            Operation::CreateAccount(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_create_account(&self) -> bool {
        self.create_account().is_some()
    }

    pub fn payment(&self) -> Option<&PaymentOperation> {
        match self {
            Operation::Payment(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_payment(&self) -> bool {
        self.payment().is_some()
    }

    pub fn path_payment_strict_receive(&self) -> Option<&PathPaymentStrictReceiveOperation> {
        match self {
            Operation::PathPaymentStrictReceive(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_path_payment_strict_receive(&self) -> bool {
        self.path_payment_strict_receive().is_some()
    }

    pub fn manage_sell_offer(&self) -> Option<&ManageSellOfferOperation> {
        match self {
            Operation::ManageSellOffer(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_manage_sell_offer(&self) -> bool {
        self.manage_sell_offer().is_some()
    }

    pub fn create_passive_sell_offer(&self) -> Option<&CreatePassiveSellOfferOperation> {
        match self {
            Operation::CreatePassiveSellOffer(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_create_passive_sell_offer(&self) -> bool {
        self.create_passive_sell_offer().is_some()
    }

    pub fn set_options(&self) -> Option<&SetOptionsOperation> {
        match self {
            Operation::SetOptions(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_set_options(&self) -> bool {
        self.set_options().is_some()
    }

    pub fn change_trust(&self) -> Option<&ChangeTrustOperation> {
        match self {
            Operation::ChangeTrust(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_change_trust(&self) -> bool {
        self.change_trust().is_some()
    }

    pub fn allow_trust(&self) -> Option<&AllowTrustOperation> {
        match self {
            Operation::AllowTrust(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_allow_trust(&self) -> bool {
        self.allow_trust().is_some()
    }

    pub fn account_merge(&self) -> Option<&AccountMergeOperation> {
        match self {
            Operation::AccountMerge(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_account_merge(&self) -> bool {
        self.account_merge().is_some()
    }

    pub fn inflation(&self) -> Option<&InflationOperation> {
        match self {
            Operation::Inflation(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_inflation(&self) -> bool {
        self.inflation().is_some()
    }

    pub fn manage_data(&self) -> Option<&ManageDataOperation> {
        match self {
            Operation::ManageData(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_manage_data(&self) -> bool {
        self.manage_data().is_some()
    }

    pub fn bump_sequence(&self) -> Option<&BumpSequenceOperation> {
        match self {
            Operation::BumpSequence(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_bump_sequence(&self) -> bool {
        self.bump_sequence().is_some()
    }

    pub fn manage_buy_offer(&self) -> Option<&ManageBuyOfferOperation> {
        match self {
            Operation::ManageBuyOffer(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_manage_buy_offer(&self) -> bool {
        self.manage_buy_offer().is_some()
    }

    pub fn path_payment_strict_send(&self) -> Option<&PathPaymentStrictSendOperation> {
        match self {
            Operation::PathPaymentStrictSend(op) => Some(op),
            _ => None,
        }
    }

    pub fn is_path_payment_strict_send(&self) -> bool {
        self.path_payment_strict_send().is_some()
    }

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
        }
    }

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
        };
        Ok(xdr::Operation {
            source_account,
            body,
        })
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::KeyPair;
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    fn keypair0() -> KeyPair {
        // GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3
        KeyPair::from_secret_seed("SBPQUZ6G4FZNWFHKUWC5BEYWF6R52E3SEP7R3GWYSM2XTKGF5LNTWW4R")
            .unwrap()
    }

    fn keypair1() -> KeyPair {
        // GAS4V4O2B7DW5T7IQRPEEVCRXMDZESKISR7DVIGKZQYYV3OSQ5SH5LVP
        KeyPair::from_secret_seed("SBMSVD4KKELKGZXHBUQTIROWUAPQASDX7KEJITARP4VMZ6KLUHOGPTYW")
            .unwrap()
    }

    fn keypair2() -> KeyPair {
        // GB7BDSZU2Y27LYNLALKKALB52WS2IZWYBDGY6EQBLEED3TJOCVMZRH7H
        KeyPair::from_secret_seed("SBZVMB74Z76QZ3ZOY7UTDFYKMEGKW5XFJEB6PFKBF4UYSSWHG4EDH7PY")
            .unwrap()
    }

    #[test]
    fn test_inflation_no_source() {
        let op = inflation().build();
        let encoded = op.xdr_base64().unwrap();
        assert_eq!("AAAAAAAAAAk=", encoded);
        let decoded = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, decoded);
    }

    #[test]
    fn test_inflation_with_source() {
        let op = inflation()
            .with_source_account(keypair0().public_key().clone())
            .build();
        let encoded = op.xdr_base64().unwrap();
        assert_eq!(
            "AAAAAQAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAAk=",
            encoded
        );
        let decoded = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, decoded);
    }
}
