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

#[derive(Debug, Default)]
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
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> ManageSellOfferOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
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

    pub fn with_amount<A>(mut self, amount: A) -> Result<ManageSellOfferOperationBuilder>
    where
        A: TryInto<Stroops, Error = Error>,
    {
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
    fn test_manage_sell_offer() {
        let kp = keypair0();
        let kp1 = keypair1();

        let amount = Amount::from_str("100.0").unwrap();
        let buying = Asset::new_credit("AB", kp1.public_key().clone()).unwrap();
        let price = Price::from_str("12.35").unwrap();

        let op = Operation::new_manage_sell_offer()
            .with_selling_asset(Asset::new_native())
            .with_buying_asset(buying)
            .with_amount(amount)
            .unwrap()
            .with_price(price)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAADAAAAAAAAAAFBQgAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAADuaygAAAAD3AAAAFAAAAAAAAAAAAAAAAAAAAAHqLnLFAAAAQPhJ8gbyJpZmlbKl5ItaNi7KAGF0BH3Dfe3ZT/f9liF1JtsEFKeoNq8ea0me2y59AH78iahdkLLxVc4oq1ROcw4=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_manage_sell_offer_with_offer_id() {
        let kp = keypair0();
        let kp1 = keypair1();

        let amount = Amount::from_str("100.0").unwrap();
        let buying = Asset::new_credit("AB", kp1.public_key().clone()).unwrap();
        let price = Price::from_str("12.35").unwrap();

        let op = Operation::new_manage_sell_offer()
            .with_selling_asset(Asset::new_native())
            .with_buying_asset(buying)
            .with_amount(amount)
            .unwrap()
            .with_price(price)
            .with_offer_id(Some(888))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAADAAAAAAAAAAFBQgAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAADuaygAAAAD3AAAAFAAAAAAAAAN4AAAAAAAAAAHqLnLFAAAAQI9ZDQtGLZFCFgqd/6dLqznGWwAI4/LOwrNS7JkO5Rbx8j1cG60rWFylW9v0i40yk7Z5HleAncBJzrvcDeHhDAA=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_manage_sell_offer_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let amount = Amount::from_str("100.0").unwrap();
        let buying = Asset::new_credit("AB", kp1.public_key().clone()).unwrap();
        let price = Price::from_str("12.35").unwrap();

        let op = Operation::new_manage_sell_offer()
            .with_source_account(kp2.public_key().clone())
            .with_selling_asset(Asset::new_native())
            .with_buying_asset(buying)
            .with_amount(amount)
            .unwrap()
            .with_price(price)
            .with_offer_id(Some(888))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAADAAAAAAAAAAFBQgAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAADuaygAAAAD3AAAAFAAAAAAAAAN4AAAAAAAAAAHqLnLFAAAAQBJhFxneImX3Z8FW+UO5xM1kAQhkwVVxSbpVqsafWaeFzcY8KWTlopVYqgoJrXnHRQq3KAstTiRktSg5zZbppgQ=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
