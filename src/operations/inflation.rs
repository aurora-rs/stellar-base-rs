use crate::crypto::MuxedAccount;
use crate::error::Result;
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InflationOperation {
    source_account: Option<MuxedAccount>,
}

#[derive(Debug)]
pub struct InflationOperationBuilder {
    source_account: Option<MuxedAccount>,
}

impl InflationOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        Ok(xdr::OperationBody::Inflation(()))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
    ) -> Result<InflationOperation> {
        Ok(InflationOperation { source_account })
    }
}

impl InflationOperationBuilder {
    pub fn new() -> InflationOperationBuilder {
        InflationOperationBuilder {
            source_account: None,
        }
    }

    pub fn with_source_account<S: Into<MuxedAccount>>(
        mut self,
        source: S,
    ) -> InflationOperationBuilder {
        self.source_account = Some(source.into());
        self
    }

    pub fn build(self) -> Operation {
        Operation::Inflation(InflationOperation {
            source_account: self.source_account,
        })
    }
}
