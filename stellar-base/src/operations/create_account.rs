use crate::amount::Stroops;
use crate::crypto::{MuxedAccount, PublicKey};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAccountOperation {
    source_account: Option<MuxedAccount>,
    destination: PublicKey,
    starting_balance: Stroops,
}

#[derive(Debug)]
pub struct CreateAccountOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<PublicKey>,
    starting_balance: Option<Stroops>,
}

impl CreateAccountOperation {
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn destination(&self) -> &PublicKey {
        &self.destination
    }

    pub fn starting_balance(&self) -> &Stroops {
        &self.starting_balance
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let destination = self.destination.to_xdr_account_id()?;
        let starting_balance = self.starting_balance.to_xdr_int64()?;
        let inner = xdr::CreateAccountOp {
            destination,
            starting_balance,
        };
        Ok(xdr::OperationBody::CreateAccount(inner))
    }

    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::CreateAccountOp,
    ) -> Result<CreateAccountOperation> {
        let destination = PublicKey::from_xdr_account_id(&x.destination)?;
        let starting_balance = Stroops::from_xdr_int64(&x.starting_balance)?;
        Ok(CreateAccountOperation {
            source_account,
            destination,
            starting_balance,
        })
    }
}

impl CreateAccountOperationBuilder {
    pub fn new() -> CreateAccountOperationBuilder {
        CreateAccountOperationBuilder {
            source_account: None,
            destination: None,
            starting_balance: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> CreateAccountOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_destination(mut self, destination: PublicKey) -> CreateAccountOperationBuilder {
        self.destination = Some(destination);
        self
    }

    pub fn with_starting_balance<B: TryInto<Stroops, Error = Error>>(
        mut self,
        starting_balance: B,
    ) -> Result<CreateAccountOperationBuilder> {
        self.starting_balance = Some(starting_balance.try_into()?);
        Ok(self)
    }

    pub fn build(self) -> Result<Operation> {
        let destination = self.destination.ok_or_else(|| {
            Error::InvalidOperation("missing create account destination".to_string())
        })?;

        let starting_balance = self.starting_balance.ok_or_else(|| {
            Error::InvalidOperation("missing create account starting balance".to_string())
        })?;

        Ok(Operation::CreateAccount(CreateAccountOperation {
            source_account: self.source_account,
            destination,
            starting_balance,
        }))
    }
}
