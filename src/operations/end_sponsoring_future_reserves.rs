use crate::crypto::MuxedAccount;
use crate::error::Result;
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndSponsoringFutureReservesOperation {
    source_account: Option<MuxedAccount>,
}

#[derive(Debug, Default)]
pub struct EndSponsoringFutureReservesOperationBuilder {
    source_account: Option<MuxedAccount>,
}

impl EndSponsoringFutureReservesOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        Ok(xdr::OperationBody::EndSponsoringFutureReserves(()))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
    ) -> Result<EndSponsoringFutureReservesOperation> {
        Ok(EndSponsoringFutureReservesOperation { source_account })
    }
}

impl EndSponsoringFutureReservesOperationBuilder {
    pub fn new() -> EndSponsoringFutureReservesOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(
        mut self,
        source: S,
    ) -> EndSponsoringFutureReservesOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn build(self) -> Operation {
        Operation::EndSponsoringFutureReserves(EndSponsoringFutureReservesOperation {
            source_account: self.source_account,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::network::Network;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_end_sponsoring_future_reserves() {
        let kp = keypair0();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(Operation::new_end_sponsoring_future_reserves().build())
            .into_transaction()
            .unwrap();
        tx.sign(&kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAARAAAAAAAAAAHqLnLFAAAAQOQlXWOVDQVXDDwpbjv6EpROZHQ77tzmTMCrczR20UFqzPBZHc6lKv9tTLzz/esVDbwYbaa+m/y8HqH7MqPjuQw=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_end_sponsoring_future_reserves_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();

        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(
                Operation::new_end_sponsoring_future_reserves()
                    .with_source_account(kp1.public_key().clone())
                    .build(),
            )
            .into_transaction()
            .unwrap();
        tx.sign(&kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAARAAAAAAAAAAHqLnLFAAAAQBjITBRZCRbpp2adVQSTC8l9NB/UoTFtHpHkZYEEWl0hSGD7U4N3m2WhFD+kmoBnyy1jWbtPMkyL3SriGvrSCwM=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
