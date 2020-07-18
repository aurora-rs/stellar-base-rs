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

    pub fn with_source_account<S>(mut self, source: S) -> CreatePassiveSellOfferOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
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

    pub fn with_amount<A>(mut self, amount: A) -> Result<CreatePassiveSellOfferOperationBuilder>
    where
        A: TryInto<Stroops>,
    {
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

#[cfg(test)]
mod tests {
    use crate::amount::{Amount, Price};
    use crate::asset::Asset;
    use crate::crypto::KeyPair;
    use crate::network::Network;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use std::str::FromStr;

    fn keypair0() -> KeyPair {
        // GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3
        KeyPair::from_secret_seed("SBPQUZ6G4FZNWFHKUWC5BEYWF6R52E3SEP7R3GWYSM2XTKGF5LNTWW4R")
            .unwrap()
    }

    fn keypair1() -> KeyPair {
        // GAS4V4O2B7DW5T7IQRPEEVCRXMDZESKISR7DVIGKZQYYV3OSQ5SH5LVP
        KeyPair::from_secret_seed("SBMSVD4KKELKGZXHBUQTIROWUAPQASDX7KEJITARP4VMZ6KLUHOGPTYW")
            .unwrap()
    }

    fn keypair2() -> KeyPair {
        // GB7BDSZU2Y27LYNLALKKALB52WS2IZWYBDGY6EQBLEED3TJOCVMZRH7H
        KeyPair::from_secret_seed("SBZVMB74Z76QZ3ZOY7UTDFYKMEGKW5XFJEB6PFKBF4UYSSWHG4EDH7PY")
            .unwrap()
    }

    #[test]
    fn test_create_passive_sell_offer() {
        let kp = keypair0();
        let kp1 = keypair1();

        let amount = Amount::from_str("100.0").unwrap();
        let buying = Asset::new_credit("AB", kp1.public_key().clone()).unwrap();
        let price = Price::from_str("12.35").unwrap();

        let op = Operation::new_create_passive_sell_offer()
            .with_selling_asset(Asset::new_native())
            .with_buying_asset(buying)
            .with_amount(amount)
            .unwrap()
            .with_price(price)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAEAAAAAAAAAAFBQgAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAADuaygAAAAD3AAAAFAAAAAAAAAAB6i5yxQAAAECG2/IOsqY2pTugmUnhX9Iafmy5JuCQjPxlA0kxdYHe2EKIbZVClMbgckEwvjJq+B0G2SzRUqiK1sfAOIZpAB4D";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_create_passive_sell_offer_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let amount = Amount::from_str("100.0").unwrap();
        let buying = Asset::new_credit("AB", kp1.public_key().clone()).unwrap();
        let price = Price::from_str("12.35").unwrap();

        let op = Operation::new_create_passive_sell_offer()
            .with_source_account(kp2.public_key().clone())
            .with_selling_asset(Asset::new_native())
            .with_buying_asset(buying)
            .with_amount(amount)
            .unwrap()
            .with_price(price)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAEAAAAAAAAAAFBQgAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAADuaygAAAAD3AAAAFAAAAAAAAAAB6i5yxQAAAECffRDt4ilFKVEfldEY/Ys2VJm4g7iu6eiqJvPGqDGALTPnEMncqaMGoFbtNgMvZWv3rXi65351/VQv1o8MrtML";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
