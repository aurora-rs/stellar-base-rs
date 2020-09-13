use crate::crypto::{MuxedAccount, PublicKey};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeginSponsoringFutureReservesOperation {
    source_account: Option<MuxedAccount>,
    sponsored_id: PublicKey,
}

#[derive(Debug, Default)]
pub struct BeginSponsoringFutureReservesOperationBuilder {
    source_account: Option<MuxedAccount>,
    sponsored_id: Option<PublicKey>,
}

impl BeginSponsoringFutureReservesOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let sponsored_id = self.sponsored_id.to_xdr_account_id()?;
        let inner = xdr::BeginSponsoringFutureReservesOp { sponsored_id };
        Ok(xdr::OperationBody::BeginSponsoringFutureReserves(inner))
    }

    /// Retrieves a reference to the sponsored id.
    pub fn sponsored_id(&self) -> &PublicKey {
        &self.sponsored_id
    }

    /// Retrieves a mutable reference to the sponsored id.
    pub fn sponsored_id_mut(&mut self) -> &mut PublicKey {
        &mut self.sponsored_id
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::BeginSponsoringFutureReservesOp,
    ) -> Result<BeginSponsoringFutureReservesOperation> {
        let sponsored_id = PublicKey::from_xdr_account_id(&x.sponsored_id)?;
        Ok(BeginSponsoringFutureReservesOperation {
            source_account,
            sponsored_id,
        })
    }
}

impl BeginSponsoringFutureReservesOperationBuilder {
    pub fn new() -> BeginSponsoringFutureReservesOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(
        mut self,
        source: S,
    ) -> BeginSponsoringFutureReservesOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_sponsored_id(
        mut self,
        sponsored_id: PublicKey,
    ) -> BeginSponsoringFutureReservesOperationBuilder {
        self.sponsored_id = Some(sponsored_id);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let sponsored_id = self.sponsored_id.ok_or_else(|| {
            Error::InvalidOperation(
                "missing begin sponsoring future reserves sponsored_id".to_string(),
            )
        })?;

        Ok(Operation::BeginSponsoringFutureReserves(
            BeginSponsoringFutureReservesOperation {
                source_account: self.source_account,
                sponsored_id,
            },
        ))
    }
}
