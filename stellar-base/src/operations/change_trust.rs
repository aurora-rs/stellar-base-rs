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
    limit: Option<i64>,
}

#[derive(Debug)]
pub struct ChangeTrustOperationBuilder {
    source_account: Option<MuxedAccount>,
    asset: Option<Asset>,
    limit: Option<i64>,
}

impl ChangeTrustOperation {
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    pub fn limit(&self) -> &Option<i64> {
        &self.limit
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let line = self.asset.to_xdr()?;
        let limit = xdr::Int64::new(self.limit.unwrap_or_else(|| 0));
        let inner = xdr::ChangeTrustOp { line, limit };
        Ok(xdr::OperationBody::ChangeTrust(inner))
    }

    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ChangeTrustOp,
    ) -> Result<ChangeTrustOperation> {
        let asset = Asset::from_xdr(&x.line)?;
        // Don't check if limit is positive because the library sure
        // has no control over the xdr.
        let limit = match &x.limit.value {
            0 => None,
            n => Some(*n),
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

    pub fn with_limit(mut self, limit: Option<i64>) -> ChangeTrustOperationBuilder {
        self.limit = limit;
        self
    }

    pub fn build(self) -> Result<Operation> {
        let asset = self
            .asset
            .ok_or_else(|| Error::InvalidOperation("missing change trust asset".to_string()))?;

        if let Some(limit) = self.limit {
            if limit <= 0 {
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
