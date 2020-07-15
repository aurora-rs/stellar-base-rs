use crate::amount::{Price, Stroops};
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManageSellOfferOperation {
    source_account: Option<MuxedAccount>,
    selling: Asset,
    buying: Asset,
    amount: Stroops,
    price: Price,
    offer_id: Option<i64>,
}

#[derive(Debug)]
pub struct ManageSellOfferOperationBuilder {
    source_account: Option<MuxedAccount>,
    selling: Option<Asset>,
    buying: Option<Asset>,
    amount: Option<Stroops>,
    price: Option<Price>,
    offer_id: Option<i64>,
}

impl ManageSellOfferOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation seling asset.
    pub fn selling(&self) -> &Asset {
        &self.selling
    }

    /// Retrieves a mutable reference to the operation seling asset.
    pub fn selling_mut(&mut self) -> &mut Asset {
        &mut self.selling
    }

    /// Retrieves the operation buying asset.
    pub fn buying(&self) -> &Asset {
        &self.buying
    }

    /// Retrieves a mutable reference to the operation buying asset.
    pub fn buying_mut(&mut self) -> &mut Asset {
        &mut self.buying
    }

    /// Retrieves the operation amount.
    pub fn amount(&self) -> &Stroops {
        &self.amount
    }

    /// Retrieves a mutable reference to the operation amount.
    pub fn amount_mut(&mut self) -> &mut Stroops {
        &mut self.amount
    }

    /// Retrieves the operation price.
    pub fn price(&self) -> &Price {
        &self.price
    }

    /// Retrieves a mutable reference to the operation price.
    pub fn price_mut(&mut self) -> &mut Price {
        &mut self.price
    }

    /// Retrieves the operation offer id.
    pub fn offer_id(&self) -> &Option<i64> {
        &self.offer_id
    }

    /// Retrieves a mutable reference the operation offer id.
    pub fn offer_id_mut(&mut self) -> &mut Option<i64> {
        &mut self.offer_id
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let selling = self.selling.to_xdr()?;
        let buying = self.buying.to_xdr()?;
        let amount = self.amount.to_xdr_int64()?;
        let price = self.price.to_xdr()?;
        let offer_id = match &self.offer_id {
            None => xdr::Int64::new(0),
            Some(id) => xdr::Int64::new(*id),
        };
        let inner = xdr::ManageSellOfferOp {
            selling,
            buying,
            amount,
            price,
            offer_id,
        };
        Ok(xdr::OperationBody::ManageSellOffer(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ManageSellOfferOp,
    ) -> Result<ManageSellOfferOperation> {
        let selling = Asset::from_xdr(&x.selling)?;
        let buying = Asset::from_xdr(&x.buying)?;
        let amount = Stroops::from_xdr_int64(&x.amount)?;
        let price = Price::from_xdr(&x.price)?;
        // Don't check if it's positive because the library user
        // has no control over the xdr.
        let offer_id = match &x.offer_id.value {
            0 => None,
            n => Some(*n),
        };
        Ok(ManageSellOfferOperation {
            source_account,
            selling,
            buying,
            amount,
            price,
            offer_id,
        })
    }
}

impl ManageSellOfferOperationBuilder {
    pub fn new() -> ManageSellOfferOperationBuilder {
        ManageSellOfferOperationBuilder {
            source_account: None,
            selling: None,
            buying: None,
            amount: None,
            price: None,
            offer_id: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> ManageSellOfferOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_selling_asset(mut self, asset: Asset) -> ManageSellOfferOperationBuilder {
        self.selling = Some(asset);
        self
    }

    pub fn with_buying_asset(mut self, asset: Asset) -> ManageSellOfferOperationBuilder {
        self.buying = Some(asset);
        self
    }

    pub fn with_amount<A: TryInto<Stroops, Error = Error>>(
        mut self,
        amount: A,
    ) -> Result<ManageSellOfferOperationBuilder> {
        self.amount = Some(amount.try_into()?);
        Ok(self)
    }

    pub fn with_price(mut self, price: Price) -> ManageSellOfferOperationBuilder {
        self.price = Some(price);
        self
    }

    pub fn with_offer_id(mut self, offer_id: Option<i64>) -> ManageSellOfferOperationBuilder {
        self.offer_id = offer_id;
        self
    }

    pub fn build(self) -> Result<Operation> {
        let selling = self.selling.ok_or_else(|| {
            Error::InvalidOperation("missing manage sell offer selling asset".to_string())
        })?;

        let buying = self.buying.ok_or_else(|| {
            Error::InvalidOperation("missing manage sell offer buying asset".to_string())
        })?;

        let amount = self.amount.ok_or_else(|| {
            Error::InvalidOperation("missing manage sell offer amount".to_string())
        })?;

        let price = self.price.ok_or_else(|| {
            Error::InvalidOperation("missing manage sell offer price".to_string())
        })?;

        if let Some(offer_id) = self.offer_id {
            if offer_id <= 0 {
                return Err(Error::InvalidOperation(
                    "manage sell offer offer_id must be positive".to_string(),
                ));
            }
        }

        Ok(Operation::ManageSellOffer(ManageSellOfferOperation {
            source_account: self.source_account,
            selling,
            buying,
            amount,
            price,
            offer_id: self.offer_id,
        }))
    }
}
