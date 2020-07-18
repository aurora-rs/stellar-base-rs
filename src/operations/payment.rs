use crate::amount::Stroops;
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentOperation {
    source_account: Option<MuxedAccount>,
    destination: MuxedAccount,
    amount: Stroops,
    asset: Asset,
}

#[derive(Debug, Default)]
pub struct PaymentOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<MuxedAccount>,
    amount: Option<Stroops>,
    asset: Option<Asset>,
}

impl PaymentOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation destination.
    pub fn destination(&self) -> &MuxedAccount {
        &self.destination
    }

    /// Retrieves a mutable reference to the operation destination.
    pub fn destination_mut(&mut self) -> &mut MuxedAccount {
        &mut self.destination
    }

    /// Retrieves the operation amount.
    pub fn amount(&self) -> &Stroops {
        &self.amount
    }

    /// Retrieves a mutable reference to the operation amount.
    pub fn amount_mut(&mut self) -> &mut Stroops {
        &mut self.amount
    }

    /// Retrieves the operation asset.
    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    /// Retrieves a mutable reference to the operation asset.
    pub fn asset_mut(&mut self) -> &mut Asset {
        &mut self.asset
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let destination = self.destination.to_xdr()?;
        let amount = self.amount.to_xdr_int64()?;
        let asset = self.asset.to_xdr()?;
        let inner = xdr::PaymentOp {
            destination,
            amount,
            asset,
        };
        Ok(xdr::OperationBody::Payment(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::PaymentOp,
    ) -> Result<PaymentOperation> {
        let destination = MuxedAccount::from_xdr(&x.destination)?;
        let asset = Asset::from_xdr(&x.asset)?;
        let amount = Stroops::from_xdr_int64(&x.amount)?;
        Ok(PaymentOperation {
            source_account,
            destination,
            asset,
            amount,
        })
    }
}

impl PaymentOperationBuilder {
    pub fn new() -> PaymentOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> PaymentOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_destination<A>(mut self, destination: A) -> PaymentOperationBuilder
    where
        A: Into<MuxedAccount>,
    {
        self.destination = Some(destination.into());
        self
    }

    pub fn with_amount<B>(mut self, amount: B) -> Result<PaymentOperationBuilder>
    where
        B: TryInto<Stroops>,
    {
        self.amount = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn with_asset(mut self, asset: Asset) -> PaymentOperationBuilder {
        self.asset = Some(asset);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let destination = self
            .destination
            .ok_or_else(|| Error::InvalidOperation("missing payment destination".to_string()))?;

        let amount = self
            .amount
            .ok_or_else(|| Error::InvalidOperation("missing payment amount".to_string()))?;

        let asset = self
            .asset
            .ok_or_else(|| Error::InvalidOperation("missing payment asset".to_string()))?;

        Ok(Operation::Payment(PaymentOperation {
            source_account: self.source_account,
            destination,
            amount,
            asset,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::Amount;
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
    fn test_payment() {
        let kp = keypair0();
        let kp1 = keypair1();
        let dest = kp1.public_key();
        let amount = Amount::from_str("12.301").unwrap();

        let op = Operation::new_payment()
            .with_destination(dest.clone())
            .with_amount(amount)
            .unwrap()
            .with_asset(Asset::new_native())
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAABAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAAAAAAAHVPvQAAAAAAAAAAHqLnLFAAAAQFOPIvnhDoRtPKJl7mJGPD69z2riRwZCJJcLRD+QaJ1Wg+yMiDHLiheBZv/BodiTqEvFHFxcmSxo7pjyzoc7mQ8=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_payment_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();
        let dest = kp1.public_key();
        let amount = Amount::from_str("12.301").unwrap();

        let op = Operation::new_payment()
            .with_source_account(kp2.public_key().clone())
            .with_destination(dest.clone())
            .with_amount(amount)
            .unwrap()
            .with_asset(Asset::new_native())
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAABAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAAAAAAAHVPvQAAAAAAAAAAHqLnLFAAAAQE6H/Nqe7xqcdepF+y5PxXJaGn6aG1xzKXDqWQajo3lsPec0puxpaFvOKs2shgjKap4BgzREPyIs9st4IKq9kAo=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
