use crate::amount::{Price, Stroops};
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeTrustOperation {
    source_account: Option<MuxedAccount>,
    asset: Asset,
    limit: Option<Stroops>,
}

#[derive(Debug)]
pub struct ChangeTrustOperationBuilder {
    source_account: Option<MuxedAccount>,
    asset: Option<Asset>,
    limit: Option<Stroops>,
}

impl ChangeTrustOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation asset.
    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    /// Retrieves a mutable reference the operation asset.
    pub fn asset_mut(&mut self) -> &mut Asset {
        &mut self.asset
    }

    /// Retrieves the operation limit.
    pub fn limit(&self) -> &Option<Stroops> {
        &self.limit
    }

    /// Retrieves a mutable reference to the operation limit.
    pub fn limit_mut(&mut self) -> &mut Option<Stroops> {
        &mut self.limit
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let line = self.asset.to_xdr()?;
        let limit = match &self.limit {
            None => xdr::Int64::new(0),
            Some(limit) => limit.to_xdr_int64()?,
        };
        let inner = xdr::ChangeTrustOp { line, limit };
        Ok(xdr::OperationBody::ChangeTrust(inner))
    }

    /// Creates from the xdr operatino body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ChangeTrustOp,
    ) -> Result<ChangeTrustOperation> {
        let asset = Asset::from_xdr(&x.line)?;
        // Don't check if limit is positive because the library sure
        // has no control over the xdr.
        let limit = match &x.limit.value {
            0 => None,
            n => Some(Stroops::new(*n)),
        };
        Ok(ChangeTrustOperation {
            source_account,
            asset,
            limit,
        })
    }
}

impl ChangeTrustOperationBuilder {
    pub fn new() -> ChangeTrustOperationBuilder {
        ChangeTrustOperationBuilder {
            source_account: None,
            asset: None,
            limit: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> ChangeTrustOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_asset(mut self, asset: Asset) -> ChangeTrustOperationBuilder {
        self.asset = Some(asset);
        self
    }

    pub fn with_limit<A: TryInto<Stroops>>(
        mut self,
        limit: Option<A>,
    ) -> Result<ChangeTrustOperationBuilder> {
        self.limit = limit
            .map(|l| l.try_into())
            .transpose()
            .map_err(|_| Error::InvalidStroopsAmount)?;
        Ok(self)
    }

    pub fn build(self) -> Result<Operation> {
        let asset = self
            .asset
            .ok_or_else(|| Error::InvalidOperation("missing change trust asset".to_string()))?;

        if let Some(limit) = &self.limit {
            if limit.to_i64() <= 0 {
                return Err(Error::InvalidOperation(
                    "change trust limit must be positive".to_string(),
                ));
            }
        }

        Ok(Operation::ChangeTrust(ChangeTrustOperation {
            source_account: self.source_account,
            asset,
            limit: self.limit,
        }))
    }
}
