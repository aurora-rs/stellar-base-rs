use crate::amount::Stroops;
use crate::crypto::{MuxedAccount, PublicKey};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAccountOperation {
    source_account: Option<MuxedAccount>,
    destination: PublicKey,
    starting_balance: Stroops,
}

#[derive(Debug, Default)]
pub struct CreateAccountOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<PublicKey>,
    starting_balance: Option<Stroops>,
}

impl CreateAccountOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation destination.
    pub fn destination(&self) -> &PublicKey {
        &self.destination
    }

    /// Retrieves a mutable reference to the operation destination.
    pub fn destination_mut(&mut self) -> &mut PublicKey {
        &mut self.destination
    }

    /// Retrieves the operation starting balance.
    pub fn starting_balance(&self) -> &Stroops {
        &self.starting_balance
    }

    /// Retrieves a mutable reference to the operation starting balance.
    pub fn starting_balance_mut(&mut self) -> &mut Stroops {
        &mut self.starting_balance
    }

    /// Returns the operation xdr body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let destination = self.destination.to_xdr_account_id()?;
        let starting_balance = self.starting_balance.to_xdr_int64()?;
        let inner = xdr::CreateAccountOp {
            destination,
            starting_balance,
        };
        Ok(xdr::OperationBody::CreateAccount(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::CreateAccountOp,
    ) -> Result<CreateAccountOperation> {
        let destination = PublicKey::from_xdr_account_id(&x.destination)?;
        let starting_balance = Stroops::from_xdr_int64(&x.starting_balance)?;
        Ok(CreateAccountOperation {
            source_account,
            destination,
            starting_balance,
        })
    }
}

impl CreateAccountOperationBuilder {
    pub fn new() -> CreateAccountOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> CreateAccountOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_destination(mut self, destination: PublicKey) -> CreateAccountOperationBuilder {
        self.destination = Some(destination);
        self
    }

    pub fn with_starting_balance<B: TryInto<Stroops>>(
        mut self,
        starting_balance: B,
    ) -> Result<CreateAccountOperationBuilder> {
        self.starting_balance = Some(
            starting_balance
                .try_into()
                .map_err(|_| Error::InvalidStroopsAmount)?,
        );
        Ok(self)
    }

    pub fn build(self) -> Result<Operation> {
        let destination = self.destination.ok_or_else(|| {
            Error::InvalidOperation("missing create account destination".to_string())
        })?;

        let starting_balance = self.starting_balance.ok_or_else(|| {
            Error::InvalidOperation("missing create account starting balance".to_string())
        })?;

        Ok(Operation::CreateAccount(CreateAccountOperation {
            source_account: self.source_account,
            destination,
            starting_balance,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::Amount;
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
    fn test_create_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let dest = kp1.public_key();
        let starting_balance = Amount::from_str("12.30").unwrap();

        let op = Operation::new_create_account()
            .with_destination(dest.clone())
            .with_starting_balance(starting_balance)
            .unwrap()
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAAdU1MAAAAAAAAAAAeoucsUAAABA0LiVS5BXQiPx/ZkMiJ55RngpeurtEgOrqbzAy99ZGnLUh68uiBejtKJdJPlw4XmVP/kojrA6nLI00zXhUiI7AQ==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_create_account_with_source() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();
        let dest = kp1.public_key();
        let starting_balance = "456.9878".parse::<Amount>().unwrap();

        let op = Operation::new_create_account()
            .with_source_account(kp2.public_key().clone())
            .with_destination(dest.clone())
            .with_starting_balance(starting_balance)
            .unwrap()
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAARBizfAAAAAAAAAAAeoucsUAAABACIZzOxwBATuDx2738HBsh0VA0oYQT1mTfrlOROtQeBb8h4AkOJeQYn3VEkij0ZnDnrZAmRP/rR7WD714PQioCA==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
