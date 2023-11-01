use crate::amount::Stroops;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::liquidity_pool::LiquidityPoolId;
use crate::{xdr, Operation};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidityPoolWithdrawOperation {
    source_account: Option<MuxedAccount>,
    liquidity_pool_id: LiquidityPoolId,
    amount: Stroops,
    min_amount_a: Stroops,
    min_amount_b: Stroops,
}

#[derive(Debug, Default)]
pub struct LiquidityPoolWithdrawOperationBuilder {
    source_account: Option<MuxedAccount>,
    liquidity_pool_id: Option<LiquidityPoolId>,
    amount: Option<Stroops>,
    min_amount_a: Option<Stroops>,
    min_amount_b: Option<Stroops>,
}

impl LiquidityPoolWithdrawOperation {
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

    pub fn amount(&self) -> &Stroops {
        &self.amount
    }

    pub fn amount_mut(&mut self) -> &mut Stroops {
        &mut self.amount
    }

    pub fn min_amount_a(&self) -> &Stroops {
        &self.min_amount_a
    }

    pub fn min_amount_a_mut(&mut self) -> &mut Stroops {
        &mut self.min_amount_a
    }

    pub fn min_amount_b(&self) -> &Stroops {
        &self.min_amount_b
    }

    pub fn min_amount_b_mut(&mut self) -> &mut Stroops {
        &mut self.min_amount_b
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inner = xdr::LiquidityPoolWithdrawOp {
            liquidity_pool_id: self.liquidity_pool_id.to_xdr(),
            amount: self.amount.to_xdr_int64()?,
            min_amount_a: self.min_amount_a.to_xdr_int64()?,
            min_amount_b: self.min_amount_b.to_xdr_int64()?,
        };
        Ok(xdr::OperationBody::LiquidityPoolWithdraw(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::LiquidityPoolWithdrawOp,
    ) -> Result<Self> {
        Ok(Self {
            source_account,
            liquidity_pool_id: LiquidityPoolId::from_xdr(&x.liquidity_pool_id)?,
            amount: Stroops::from_xdr_int64(&x.amount)?,
            min_amount_a: Stroops::from_xdr_int64(&x.min_amount_a)?,
            min_amount_b: Stroops::from_xdr_int64(&x.min_amount_b)?,
        })
    }
}

impl LiquidityPoolWithdrawOperationBuilder {
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

    pub fn with_amount<S>(mut self, amount: S) -> Result<Self>
    where
        S: TryInto<Stroops>,
    {
        self.amount = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn with_min_amount_a<S>(mut self, amount: S) -> Result<Self>
    where
        S: TryInto<Stroops>,
    {
        self.min_amount_a = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn with_min_amount_b<S>(mut self, amount: S) -> Result<Self>
    where
        S: TryInto<Stroops>,
    {
        self.min_amount_b = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn build(self) -> Result<Operation> {
        let liquidity_pool_id = self.liquidity_pool_id.ok_or_else(|| {
            Error::InvalidOperation(
                "missing liquidity pool id for liquidity pool withdraw operation".to_string(),
            )
        })?;

        let amount = self.amount.ok_or_else(|| {
            Error::InvalidOperation(
                "missing amount for liquidity pool withdraw operation".to_string(),
            )
        })?;

        let min_amount_a = self.min_amount_a.ok_or_else(|| {
            Error::InvalidOperation(
                "missing min amount for asset A for liquidity pool withdraw operation".to_string(),
            )
        })?;

        let min_amount_b = self.min_amount_b.ok_or_else(|| {
            Error::InvalidOperation(
                "missing min amount for asset B for liquidity pool withdraw operation".to_string(),
            )
        })?;

        Ok(Operation::LiquidityPoolWithdraw(
            LiquidityPoolWithdrawOperation {
                source_account: self.source_account,
                liquidity_pool_id,
                amount,
                min_amount_a,
                min_amount_b,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::Amount;
    use crate::liquidity_pool::LiquidityPoolId;
    use crate::operations::tests::*;
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use crate::Operation;
    use std::str::FromStr;

    #[test]
    fn test_liquidity_pool_withdraw() {
        let liquidity_pool_id = LiquidityPoolId::new(vec![7; 32]).unwrap();
        let amount = Amount::from_str("100.09").unwrap();
        let min_amount_a = Amount::from_str("1700.69").unwrap();
        let min_amount_b = Amount::from_str("6969.444").unwrap();

        let op = Operation::new_liquidity_pool_withdraw()
            .with_liquidity_pool_id(liquidity_pool_id)
            .with_amount(amount)
            .unwrap()
            .with_min_amount_a(min_amount_a)
            .unwrap()
            .with_min_amount_b(min_amount_b)
            .unwrap()
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let expected = "AAAAAAAAABcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwAAAAA7qIWgAAAAA/WwsyAAAAAQOhzCQA==";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, back);
    }

    #[test]
    fn test_liquidity_pool_withdraw_with_source_account() {
        let source_account = keypair0().public_key().clone();
        let liquidity_pool_id = LiquidityPoolId::new(vec![7; 32]).unwrap();
        let amount = Amount::from_str("100.09").unwrap();
        let min_amount_a = Amount::from_str("1700.69").unwrap();
        let min_amount_b = Amount::from_str("6969.444").unwrap();

        let op = Operation::new_liquidity_pool_withdraw()
            .with_source_account(source_account)
            .with_liquidity_pool_id(liquidity_pool_id)
            .with_amount(amount)
            .unwrap()
            .with_min_amount_a(min_amount_a)
            .unwrap()
            .with_min_amount_b(min_amount_b)
            .unwrap()
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let expected = "AAAAAQAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAABcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwAAAAA7qIWgAAAAA/WwsyAAAAAQOhzCQA==";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, back);
    }
}
