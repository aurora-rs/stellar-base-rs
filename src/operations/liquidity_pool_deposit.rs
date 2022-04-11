use crate::amount::{Price, Stroops};
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::liquidity_pool::LiquidityPoolId;
use crate::{xdr, Operation};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidityPoolDepositOperation {
    source_account: Option<MuxedAccount>,
    liquidity_pool_id: LiquidityPoolId,
    max_amount_a: Stroops,
    max_amount_b: Stroops,
    min_price: Price,
    max_price: Price,
}

#[derive(Debug, Default)]
pub struct LiquidityPoolDepositOperationBuilder {
    source_account: Option<MuxedAccount>,
    liquidity_pool_id: Option<LiquidityPoolId>,
    max_amount_a: Option<Stroops>,
    max_amount_b: Option<Stroops>,
    min_price: Option<Price>,
    max_price: Option<Price>,
}

impl LiquidityPoolDepositOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    pub fn liquidity_pool_id(&self) -> &LiquidityPoolId {
        &self.liquidity_pool_id
    }

    pub fn liquidity_pool_id_mut(&mut self) -> &mut LiquidityPoolId {
        &mut self.liquidity_pool_id
    }

    pub fn max_amount_a(&self) -> &Stroops {
        &self.max_amount_a
    }

    pub fn max_amount_a_mut(&mut self) -> &Stroops {
        &mut self.max_amount_a
    }

    pub fn max_amount_b(&self) -> &Stroops {
        &self.max_amount_b
    }

    pub fn max_amount_b_mut(&mut self) -> &Stroops {
        &mut self.max_amount_b
    }

    pub fn min_price(&self) -> &Price {
        &self.min_price
    }

    pub fn min_price_mut(&mut self) -> &Price {
        &mut self.min_price
    }

    pub fn max_price(&self) -> &Price {
        &self.max_price
    }

    pub fn max_price_mut(&mut self) -> &Price {
        &mut self.max_price
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inner = xdr::LiquidityPoolDepositOp {
            liquidity_pool_id: self.liquidity_pool_id.to_xdr(),
            max_amount_a: self.max_amount_a.to_xdr_int64()?,
            max_amount_b: self.max_amount_b.to_xdr_int64()?,
            min_price: self.min_price.to_xdr()?,
            max_price: self.max_price.to_xdr()?,
        };
        Ok(xdr::OperationBody::LiquidityPoolDeposit(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::LiquidityPoolDepositOp,
    ) -> Result<Self> {
        Ok(Self {
            source_account,
            liquidity_pool_id: LiquidityPoolId::from_xdr(&x.liquidity_pool_id)?,
            max_amount_a: Stroops::from_xdr_int64(&x.max_amount_a)?,
            max_amount_b: Stroops::from_xdr_int64(&x.max_amount_b)?,
            min_price: Price::from_xdr(&x.min_price)?,
            max_price: Price::from_xdr(&x.max_price)?,
        })
    }
}

impl LiquidityPoolDepositOperationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> Self
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_liquidity_pool_id(mut self, liquidity_pool_id: LiquidityPoolId) -> Self {
        self.liquidity_pool_id = Some(liquidity_pool_id);
        self
    }

    pub fn with_max_amount_a<S>(mut self, amount: S) -> Result<Self>
    where
        S: TryInto<Stroops>,
    {
        self.max_amount_a = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn with_max_amount_b<S>(mut self, amount: S) -> Result<Self>
    where
        S: TryInto<Stroops>,
    {
        self.max_amount_b = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn with_min_price(mut self, price: Price) -> Self {
        self.min_price = Some(price);
        self
    }

    pub fn with_max_price(mut self, price: Price) -> Self {
        self.max_price = Some(price);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let liquidity_pool_id = self.liquidity_pool_id.ok_or_else(|| {
            Error::InvalidOperation(
                "missing liquidity pool id for liquidity pool deposit operation".to_string(),
            )
        })?;

        let max_amount_a = self.max_amount_a.ok_or_else(|| {
            Error::InvalidOperation(
                "missing max amount for asset A for liquidity pool deposit operation".to_string(),
            )
        })?;

        let max_amount_b = self.max_amount_b.ok_or_else(|| {
            Error::InvalidOperation(
                "missing max amount for asset B for liquidity pool deposit operation".to_string(),
            )
        })?;

        let min_price = self.min_price.ok_or_else(|| {
            Error::InvalidOperation(
                "missing min price for liquidity pool deposit operation".to_string(),
            )
        })?;

        let max_price = self.max_price.ok_or_else(|| {
            Error::InvalidOperation(
                "missing max price for liquidity pool deposit operation".to_string(),
            )
        })?;

        Ok(Operation::LiquidityPoolDeposit(
            LiquidityPoolDepositOperation {
                source_account: self.source_account,
                liquidity_pool_id,
                max_amount_a,
                max_amount_b,
                min_price,
                max_price,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::{Amount, Price};
    use crate::liquidity_pool::LiquidityPoolId;
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use crate::{KeyPair, Operation};
    use std::str::FromStr;

    fn keypair0() -> KeyPair {
        // GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3
        KeyPair::from_secret_seed("SBPQUZ6G4FZNWFHKUWC5BEYWF6R52E3SEP7R3GWYSM2XTKGF5LNTWW4R")
            .unwrap()
    }

    #[test]
    fn test_liquidity_pool_deposit() {
        let liquidity_pool_id = LiquidityPoolId::new(vec![7; 32]).unwrap();
        let max_amount_a = Amount::from_str("1700.69").unwrap();
        let max_amount_b = Amount::from_str("6969.444").unwrap();
        let min_price = Price::from_str("1.00").unwrap();
        let max_price = Price::from_str("25.25").unwrap();

        let op = Operation::new_liquidity_pool_deposit()
            .with_liquidity_pool_id(liquidity_pool_id)
            .with_max_amount_a(max_amount_a)
            .unwrap()
            .with_max_amount_b(max_amount_b)
            .unwrap()
            .with_min_price(min_price)
            .with_max_price(max_price)
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let expected = "AAAAAAAAABYHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwAAAAP1sLMgAAAAEDocwkAAAAABAAAAAQAAAGUAAAAE";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, back);
    }

    #[test]
    fn test_liquidity_pool_deposit_with_source_account() {
        let source_account = keypair0();
        let liquidity_pool_id = LiquidityPoolId::new(vec![7; 32]).unwrap();
        let max_amount_a = Amount::from_str("1700.69").unwrap();
        let max_amount_b = Amount::from_str("6969.444").unwrap();
        let min_price = Price::from_str("1.00").unwrap();
        let max_price = Price::from_str("25.25").unwrap();

        let op = Operation::new_liquidity_pool_deposit()
            .with_source_account(source_account.public_key().clone())
            .with_liquidity_pool_id(liquidity_pool_id)
            .with_max_amount_a(max_amount_a)
            .unwrap()
            .with_max_amount_b(max_amount_b)
            .unwrap()
            .with_min_price(min_price)
            .with_max_price(max_price)
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let expected = "AAAAAQAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAABYHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwAAAAP1sLMgAAAAEDocwkAAAAABAAAAAQAAAGUAAAAE";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, back);
    }
}
