use crate::amount::{Price, Stroops};
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePassiveSellOfferOperation {
    source_account: Option<MuxedAccount>,
    selling: Asset,
    buying: Asset,
    amount: Stroops,
    price: Price,
}

#[derive(Debug)]
pub struct CreatePassiveSellOfferOperationBuilder {
    source_account: Option<MuxedAccount>,
    selling: Option<Asset>,
    buying: Option<Asset>,
    amount: Option<Stroops>,
    price: Option<Price>,
}

impl CreatePassiveSellOfferOperation {
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn selling(&self) -> &Asset {
        &self.selling
    }

    pub fn buying(&self) -> &Asset {
        &self.buying
    }

    pub fn amount(&self) -> &Stroops {
        &self.amount
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let selling = self.selling.to_xdr()?;
        let buying = self.buying.to_xdr()?;
        let amount = self.amount.to_xdr_int64()?;
        let price = self.price.to_xdr()?;
        let inner = xdr::CreatePassiveSellOfferOp {
            selling,
            buying,
            amount,
            price,
        };
        Ok(xdr::OperationBody::CreatePassiveSellOffer(inner))
    }

    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::CreatePassiveSellOfferOp,
    ) -> Result<CreatePassiveSellOfferOperation> {
        let selling = Asset::from_xdr(&x.selling)?;
        let buying = Asset::from_xdr(&x.buying)?;
        let amount = Stroops::from_xdr_int64(&x.amount)?;
        let price = Price::from_xdr(&x.price)?;
        Ok(CreatePassiveSellOfferOperation {
            source_account,
            selling,
            buying,
            amount,
            price,
        })
    }
}

impl CreatePassiveSellOfferOperationBuilder {
    pub fn new() -> CreatePassiveSellOfferOperationBuilder {
        CreatePassiveSellOfferOperationBuilder {
            source_account: None,
            selling: None,
            buying: None,
            amount: None,
            price: None,
        }
    }

    pub fn with_source_account(
        mut self,
        source: MuxedAccount,
    ) -> CreatePassiveSellOfferOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_selling_asset(mut self, asset: Asset) -> CreatePassiveSellOfferOperationBuilder {
        self.selling = Some(asset);
        self
    }

    pub fn with_buying_asset(mut self, asset: Asset) -> CreatePassiveSellOfferOperationBuilder {
        self.buying = Some(asset);
        self
    }

    pub fn with_amount<A: TryInto<Stroops, Error = Error>>(
        mut self,
        amount: A,
    ) -> Result<CreatePassiveSellOfferOperationBuilder> {
        self.amount = Some(amount.try_into()?);
        Ok(self)
    }

    pub fn with_price(mut self, price: Price) -> CreatePassiveSellOfferOperationBuilder {
        self.price = Some(price);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let selling = self.selling.ok_or_else(|| {
            Error::InvalidOperation("missing create passive sell offer selling asset".to_string())
        })?;

        let buying = self.buying.ok_or_else(|| {
            Error::InvalidOperation("missing create passive sell offer buying asset".to_string())
        })?;

        let amount = self.amount.ok_or_else(|| {
            Error::InvalidOperation("missing create passive sell offer amount".to_string())
        })?;

        let price = self.price.ok_or_else(|| {
            Error::InvalidOperation("missing create passive sell offer price".to_string())
        })?;

        Ok(Operation::CreatePassiveSellOffer(
            CreatePassiveSellOfferOperation {
                source_account: self.source_account,
                selling,
                buying,
                amount,
                price,
            },
        ))
    }
}
