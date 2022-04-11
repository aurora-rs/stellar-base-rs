use crate::crypto::{MuxedAccount, PublicKey};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeginSponsoringFutureReservesOperation {
    source_account: Option<MuxedAccount>,
    sponsored_id: PublicKey,
}

#[derive(Debug, Default)]
pub struct BeginSponsoringFutureReservesOperationBuilder {
    source_account: Option<MuxedAccount>,
    sponsored_id: Option<PublicKey>,
}

impl BeginSponsoringFutureReservesOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves a reference to the sponsored id.
    pub fn sponsored_id(&self) -> &PublicKey {
        &self.sponsored_id
    }

    /// Retrieves a mutable reference to the sponsored id.
    pub fn sponsored_id_mut(&mut self) -> &mut PublicKey {
        &mut self.sponsored_id
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let sponsored_id = self.sponsored_id.to_xdr_account_id()?;
        let inner = xdr::BeginSponsoringFutureReservesOp { sponsored_id };
        Ok(xdr::OperationBody::BeginSponsoringFutureReserves(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::BeginSponsoringFutureReservesOp,
    ) -> Result<BeginSponsoringFutureReservesOperation> {
        let sponsored_id = PublicKey::from_xdr_account_id(&x.sponsored_id)?;
        Ok(BeginSponsoringFutureReservesOperation {
            source_account,
            sponsored_id,
        })
    }
}

impl BeginSponsoringFutureReservesOperationBuilder {
    pub fn new() -> BeginSponsoringFutureReservesOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(
        mut self,
        source: S,
    ) -> BeginSponsoringFutureReservesOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_sponsored_id(
        mut self,
        sponsored_id: PublicKey,
    ) -> BeginSponsoringFutureReservesOperationBuilder {
        self.sponsored_id = Some(sponsored_id);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let sponsored_id = self.sponsored_id.ok_or_else(|| {
            Error::InvalidOperation(
                "missing begin sponsoring future reserves sponsored_id".to_string(),
            )
        })?;

        Ok(Operation::BeginSponsoringFutureReserves(
            BeginSponsoringFutureReservesOperation {
                source_account: self.source_account,
                sponsored_id,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::KeyPair;
    use crate::network::Network;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};

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
    fn test_begin_sponsoring_future_reserves() {
        let kp = keypair0();
        let kp1 = keypair1();

        let op = Operation::new_begin_sponsoring_future_reserves()
            .with_sponsored_id(kp1.public_key().clone())
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAQAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAAAAAAHqLnLFAAAAQIf45R2LIdRhc5dYfL6gFHyeBmxaIMfts2/+oOOhyVyKXYShySjM3xR9k1dCbg7cmgGJAmPo49RZczTXE1Ufjgs=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_begin_sponsoring_future_reserves_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let op = Operation::new_begin_sponsoring_future_reserves()
            .with_source_account(kp2.public_key().clone())
            .with_sponsored_id(kp1.public_key().clone())
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAQAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAAAAAAHqLnLFAAAAQCxAVR1PtQyNc3Bin78gpQlhX+Gu6r9RWh7Rezak0wJwOebKRKsjdO0R3hvQN9ZwSAQDeF5Du4panJMptWkgLQo=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
