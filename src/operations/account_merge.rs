use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountMergeOperation {
    source_account: Option<MuxedAccount>,
    destination: MuxedAccount,
}

#[derive(Debug)]
pub struct AccountMergeOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<MuxedAccount>,
}

impl AccountMergeOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieve the operation destination.
    pub fn destination(&self) -> &MuxedAccount {
        &self.destination
    }

    /// Retrieve the operation destination.
    pub fn destination_mut(&mut self) -> &mut MuxedAccount {
        &mut self.destination
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let muxed_account = self.destination.to_xdr()?;
        Ok(xdr::OperationBody::AccountMerge(muxed_account))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::MuxedAccount,
    ) -> Result<AccountMergeOperation> {
        let destination = MuxedAccount::from_xdr(x)?;
        Ok(AccountMergeOperation {
            source_account,
            destination,
        })
    }
}

impl AccountMergeOperationBuilder {
    pub fn new() -> AccountMergeOperationBuilder {
        AccountMergeOperationBuilder {
            source_account: None,
            destination: None,
        }
    }

    pub fn with_source_account<S>(mut self, source: S) -> AccountMergeOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_destination(mut self, destination: MuxedAccount) -> AccountMergeOperationBuilder {
        self.destination = Some(destination);
        self
    }

    pub fn build(mut self) -> Result<Operation> {
        if let Some(destination) = self.destination.take() {
            Ok(Operation::AccountMerge(AccountMergeOperation {
                source_account: self.source_account,
                destination,
            }))
        } else {
            Err(Error::InvalidOperation(
                "missing account merge destination".to_string(),
            ))
        }
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
    fn test_account_merge() {
        let kp = keypair0();
        let kp1 = keypair1();

        let op = Operation::new_account_merge()
            .with_destination(kp1.public_key().to_muxed_account(123).clone())
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAIAAABAAAAAAAAAAB7Jcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAAAAAAAAeoucsUAAABAsar7y6Xy4F1R8InkdOL21n646Z5Fe6dUUfhAkDf9GnIQKHcFzVoeo7z73S6V0zW1AxfR/wJDsFprWPhbllYhBw==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_account_merge_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let op = Operation::new_account_merge()
            .with_destination(kp1.public_key().to_muxed_account(123).clone())
            .with_source_account(kp2.public_key().clone())
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAIAAABAAAAAAAAAAB7Jcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAAAAAAAAeoucsUAAABAXZWyIftD9wZB7iI/CPWdye29ggvdwf20/WQr00IbTshuas0JDIbJhhfK8NUrEILhBspQjRx82XCrppZVrampAQ==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
