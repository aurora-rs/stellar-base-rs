use crate::amount::{Price, Stroops};
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManageBuyOfferOperation {
    source_account: Option<MuxedAccount>,
    selling: Asset,
    buying: Asset,
    buy_amount: Stroops,
    price: Price,
    offer_id: Option<i64>,
}

#[derive(Debug)]
pub struct ManageBuyOfferOperationBuilder {
    source_account: Option<MuxedAccount>,
    selling: Option<Asset>,
    buying: Option<Asset>,
    buy_amount: Option<Stroops>,
    price: Option<Price>,
    offer_id: Option<i64>,
}

impl ManageBuyOfferOperation {
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn selling(&self) -> &Asset {
        &self.selling
    }

    pub fn buying(&self) -> &Asset {
        &self.buying
    }

    pub fn buy_amount(&self) -> &Stroops {
        &self.buy_amount
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn offer_id(&self) -> &Option<i64> {
        &self.offer_id
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let selling = self.selling.to_xdr()?;
        let buying = self.buying.to_xdr()?;
        let buy_amount = self.buy_amount.to_xdr_int64()?;
        let price = self.price.to_xdr()?;
        let offer_id = match &self.offer_id {
            None => xdr::Int64::new(0),
            Some(id) => xdr::Int64::new(*id),
        };
        let inner = xdr::ManageBuyOfferOp {
            selling,
            buying,
            buy_amount,
            price,
            offer_id,
        };
        Ok(xdr::OperationBody::ManageBuyOffer(inner))
    }

    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ManageBuyOfferOp,
    ) -> Result<ManageBuyOfferOperation> {
        let selling = Asset::from_xdr(&x.selling)?;
        let buying = Asset::from_xdr(&x.buying)?;
        let buy_amount = Stroops::from_xdr_int64(&x.buy_amount)?;
        let price = Price::from_xdr(&x.price)?;
        // Don't check if it's positive because the library user
        // has no control over the xdr.
        let offer_id = match &x.offer_id.value {
            0 => None,
            n => Some(*n),
        };
        Ok(ManageBuyOfferOperation {
            source_account,
            selling,
            buying,
            buy_amount,
            price,
            offer_id,
        })
    }
}

impl ManageBuyOfferOperationBuilder {
    pub fn new() -> ManageBuyOfferOperationBuilder {
        ManageBuyOfferOperationBuilder {
            source_account: None,
            selling: None,
            buying: None,
            buy_amount: None,
            price: None,
            offer_id: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> ManageBuyOfferOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_selling_asset(mut self, asset: Asset) -> ManageBuyOfferOperationBuilder {
        self.selling = Some(asset);
        self
    }

    pub fn with_buying_asset(mut self, asset: Asset) -> ManageBuyOfferOperationBuilder {
        self.buying = Some(asset);
        self
    }

    pub fn with_buy_amount<A: TryInto<Stroops, Error = Error>>(
        mut self,
        amount: A,
    ) -> Result<ManageBuyOfferOperationBuilder> {
        self.buy_amount = Some(amount.try_into()?);
        Ok(self)
    }

    pub fn with_price(mut self, price: Price) -> ManageBuyOfferOperationBuilder {
        self.price = Some(price);
        self
    }

    pub fn with_offer_id(mut self, offer_id: Option<i64>) -> ManageBuyOfferOperationBuilder {
        self.offer_id = offer_id;
        self
    }

    pub fn build(self) -> Result<Operation> {
        let selling = self.selling.ok_or_else(|| {
            Error::InvalidOperation("missing manage buy offer selling asset".to_string())
        })?;

        let buying = self.buying.ok_or_else(|| {
            Error::InvalidOperation("missing manage buy offer buying asset".to_string())
        })?;

        let buy_amount = self.buy_amount.ok_or_else(|| {
            Error::InvalidOperation("missing manage buy offer amount".to_string())
        })?;

        let price = self
            .price
            .ok_or_else(|| Error::InvalidOperation("missing manage buy offer price".to_string()))?;

        if let Some(offer_id) = self.offer_id {
            if offer_id <= 0 {
                return Err(Error::InvalidOperation(
                    "manage buy offer offer_id must be positive".to_string(),
                ));
            }
        }

        Ok(Operation::ManageBuyOffer(ManageBuyOfferOperation {
            source_account: self.source_account,
            selling,
            buying,
            buy_amount,
            price,
            offer_id: self.offer_id,
        }))
    }
}
