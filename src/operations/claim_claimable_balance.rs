use crate::claim::ClaimableBalanceId;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimClaimableBalanceOperation {
    source_account: Option<MuxedAccount>,
    balance_id: ClaimableBalanceId,
}

#[derive(Debug, Default)]
pub struct ClaimClaimableBalanceOperationBuilder {
    source_account: Option<MuxedAccount>,
    balance_id: Option<ClaimableBalanceId>,
}

impl ClaimClaimableBalanceOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves a reference to the claimable balance id.
    pub fn balance_id(&self) -> &ClaimableBalanceId {
        &self.balance_id
    }

    /// Retrieves a mutable reference to the claimable balance id.
    pub fn balance_id_mut(&mut self) -> &mut ClaimableBalanceId {
        &mut self.balance_id
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let balance_id = self.balance_id.to_xdr();
        let inner = xdr::ClaimClaimableBalanceOp { balance_id };
        Ok(xdr::OperationBody::ClaimClaimableBalance(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ClaimClaimableBalanceOp,
    ) -> Result<ClaimClaimableBalanceOperation> {
        let balance_id = ClaimableBalanceId::from_xdr(&x.balance_id)?;
        Ok(ClaimClaimableBalanceOperation {
            source_account,
            balance_id,
        })
    }
}

impl ClaimClaimableBalanceOperationBuilder {
    pub fn new() -> ClaimClaimableBalanceOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> ClaimClaimableBalanceOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_claimable_balance_id(
        mut self,
        balance_id: ClaimableBalanceId,
    ) -> ClaimClaimableBalanceOperationBuilder {
        self.balance_id = Some(balance_id);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let balance_id = self.balance_id.ok_or_else(|| {
            Error::InvalidOperation("missing claim claimable balance id".to_string())
        })?;

        Ok(Operation::ClaimClaimableBalance(
            ClaimClaimableBalanceOperation {
                source_account: self.source_account,
                balance_id,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::claim::ClaimableBalanceId;

    use crate::network::Network;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_claim_claimable_balance() {
        let kp = keypair0();

        let balance_id = ClaimableBalanceId::new(vec![7; 32]).unwrap();

        let op = Operation::new_claim_claimable_balance()
            .with_claimable_balance_id(balance_id)
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAPAAAAAAcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHAAAAAAAAAAHqLnLFAAAAQH9SMMxYh28+t9WP0xrJWUZH/i7mk9jG1Qzjvnh2I2Hx/M8OTOqr0511hvBuHI+ETJiAEMUoFw8zb16fXHjQoQQ=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_claim_claimable_balance_with_source_account() {
        let kp = keypair0();

        let balance_id = ClaimableBalanceId::new(vec![7; 32]).unwrap();

        let op = Operation::new_claim_claimable_balance()
            .with_source_account(kp.public_key())
            .with_claimable_balance_id(balance_id)
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAA4Nxt4XJcrGZRYrUvrOc1sooiQ+QdEk1suS1wo+oucsUAAAAPAAAAAAcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHAAAAAAAAAAHqLnLFAAAAQD80Q+16BK4ZzJzNGaesezr0cPxsyOvgVVlANU1g91tfM6/NkzLXkB1QI2F4xkXIbp5qbSJ/OfrGRxmsdwWo2g0=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
