use crate::amount::Stroops;
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentOperation {
    source_account: Option<MuxedAccount>,
    destination: MuxedAccount,
    amount: Stroops,
    asset: Asset,
}

#[derive(Debug)]
pub struct PaymentOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<MuxedAccount>,
    amount: Option<Stroops>,
    asset: Option<Asset>,
}

impl PaymentOperation {
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn destination(&self) -> &MuxedAccount {
        &self.destination
    }

    pub fn amount(&self) -> &Stroops {
        &self.amount
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let destination = self.destination.to_xdr()?;
        let amount = self.amount.to_xdr_int64()?;
        let asset = self.asset.to_xdr()?;
        let inner = xdr::PaymentOp {
            destination,
            amount,
            asset,
        };
        Ok(xdr::OperationBody::Payment(inner))
    }

    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::PaymentOp,
    ) -> Result<PaymentOperation> {
        let destination = MuxedAccount::from_xdr(&x.destination)?;
        let asset = Asset::from_xdr(&x.asset)?;
        let amount = Stroops::from_xdr_int64(&x.amount)?;
        Ok(PaymentOperation {
            source_account,
            destination,
            asset,
            amount,
        })
    }
}

impl PaymentOperationBuilder {
    pub fn new() -> PaymentOperationBuilder {
        PaymentOperationBuilder {
            source_account: None,
            destination: None,
            amount: None,
            asset: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> PaymentOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_destination<A: Into<MuxedAccount>>(
        mut self,
        destination: A,
    ) -> PaymentOperationBuilder {
        self.destination = Some(destination.into());
        self
    }

    pub fn with_amount<B: TryInto<Stroops>>(
        mut self,
        amount: B,
    ) -> Result<PaymentOperationBuilder> {
        self.amount = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn with_asset(mut self, asset: Asset) -> PaymentOperationBuilder {
        self.asset = Some(asset);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let destination = self
            .destination
            .ok_or_else(|| Error::InvalidOperation("missing payment destination".to_string()))?;

        let amount = self
            .amount
            .ok_or_else(|| Error::InvalidOperation("missing payment amount".to_string()))?;

        let asset = self
            .asset
            .ok_or_else(|| Error::InvalidOperation("missing payment asset".to_string()))?;

        Ok(Operation::Payment(PaymentOperation {
            source_account: self.source_account,
            destination,
            amount,
            asset,
        }))
    }
}
