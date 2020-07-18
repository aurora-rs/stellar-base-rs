use crate::crypto::MuxedAccount;
use crate::error::Result;
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InflationOperation {
    source_account: Option<MuxedAccount>,
}

#[derive(Debug)]
pub struct InflationOperationBuilder {
    source_account: Option<MuxedAccount>,
}

impl InflationOperation {
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
        Ok(xdr::OperationBody::Inflation(()))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
    ) -> Result<InflationOperation> {
        Ok(InflationOperation { source_account })
    }
}

impl InflationOperationBuilder {
    pub fn new() -> InflationOperationBuilder {
        InflationOperationBuilder {
            source_account: None,
        }
    }

    pub fn with_source_account<S>(mut self, source: S) -> InflationOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn build(self) -> Operation {
        Operation::Inflation(InflationOperation {
            source_account: self.source_account,
        })
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

    #[test]
    fn test_inflation() {
        let kp = keypair0();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(Operation::new_inflation().build())
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAJAAAAAAAAAAHqLnLFAAAAQCvHHPKuTRaRXk9BH05oWii0PJRmVOoqMxxg+79MLO90n1ljVNoaQ1Fliy8Xe34yfUzjhMB/TCXH29T8dTYtBg4=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_inflation_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();

        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(
                Operation::new_inflation()
                    .with_source_account(kp1.public_key().clone())
                    .build(),
            )
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAAJAAAAAAAAAAHqLnLFAAAAQEMnHq75AT55x0OCFN8mQGbwamRmSdlZsOw0U9z5SbhZqug7qlrW4R7U4era7DDGwOv1bdIMCcGZ4FxSDMH8hwg=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
