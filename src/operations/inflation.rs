use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
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
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        Ok(xdr::OperationBody::Inflation(()))
    }

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
