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

    /// Retrieves the operation amout.
    pub fn amount(&self) -> &Stroops {
        &self.amount
    }

    /// Retrieves a mutable reference to the operation amout.
    pub fn amount_mut(&mut self) -> &mut Stroops {
        &mut self.amount
    }

    /// Retrieves the operation price.
    pub fn price(&self) -> &Price {
        &self.price
    }

    /// Retrieves a mutable reference to the operation price.
    pub fn price_(&mut self) -> &mut Price {
        &mut self.price
    }

    /// Returns the xdr operation body.
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

    /// Creates from the xdr operation body.
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

    pub fn with_amount<A: TryInto<Stroops>>(
        mut self,
        amount: A,
    ) -> Result<CreatePassiveSellOfferOperationBuilder> {
        self.amount = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
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
