use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountMergeOperation {
    source_account: Option<MuxedAccount>,
    destination: MuxedAccount,
}

#[derive(Debug)]
pub struct AccountMergeOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<MuxedAccount>,
}

impl AccountMergeOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieve the operation destination.
    pub fn destination(&self) -> &MuxedAccount {
        &self.destination
    }

    /// Retrieve the operation destination.
    pub fn destination_mut(&mut self) -> &mut MuxedAccount {
        &mut self.destination
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let muxed_account = self.destination.to_xdr()?;
        Ok(xdr::OperationBody::AccountMerge(muxed_account))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::MuxedAccount,
    ) -> Result<AccountMergeOperation> {
        let destination = MuxedAccount::from_xdr(x)?;
        Ok(AccountMergeOperation {
            source_account,
            destination,
        })
    }
}

impl AccountMergeOperationBuilder {
    pub fn new() -> AccountMergeOperationBuilder {
        AccountMergeOperationBuilder {
            source_account: None,
            destination: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> AccountMergeOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn build(mut self) -> Result<Operation> {
        if let Some(destination) = self.destination.take() {
            Ok(Operation::AccountMerge(AccountMergeOperation {
                source_account: self.source_account,
                destination,
            }))
        } else {
            Err(Error::InvalidOperation(
                "missing account merge destination".to_string(),
            ))
        }
    }
}
