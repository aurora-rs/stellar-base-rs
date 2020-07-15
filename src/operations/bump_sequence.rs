use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BumpSequenceOperation {
    source_account: Option<MuxedAccount>,
    bump_to: i64,
}

#[derive(Debug)]
pub struct BumpSequenceOperationBuilder {
    source_account: Option<MuxedAccount>,
    bump_to: Option<i64>,
}

impl BumpSequenceOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operatino bump to.
    pub fn bump_to(&self) -> &i64 {
        &self.bump_to
    }

    /// Retrieves the operatino bump to.
    pub fn bump_to_mut(&mut self) -> &mut i64 {
        &mut self.bump_to
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let bump_to = xdr::SequenceNumber::new(xdr::Int64::new(self.bump_to));
        let inner = xdr::BumpSequenceOp { bump_to };
        Ok(xdr::OperationBody::BumpSequence(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::BumpSequenceOp,
    ) -> Result<BumpSequenceOperation> {
        let bump_to = x.bump_to.value.value;
        Ok(BumpSequenceOperation {
            source_account,
            bump_to,
        })
    }
}

impl BumpSequenceOperationBuilder {
    pub fn new() -> BumpSequenceOperationBuilder {
        BumpSequenceOperationBuilder {
            source_account: None,
            bump_to: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> BumpSequenceOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_bump_to(mut self, bump_to: i64) -> BumpSequenceOperationBuilder {
        self.bump_to = Some(bump_to);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let bump_to = self
            .bump_to
            .ok_or_else(|| Error::InvalidOperation("missing bump sequence bump to".to_string()))?;

        if bump_to < 0 {
            return Err(Error::InvalidOperation(
                "bump sequence bump to must be non negative".to_string(),
            ));
        }

        Ok(Operation::BumpSequence(BumpSequenceOperation {
            source_account: self.source_account,
            bump_to,
        }))
    }
}
