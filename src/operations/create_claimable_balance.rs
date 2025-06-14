use crate::amount::Stroops;
use crate::asset::Asset;
use crate::claim::Claimant;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateClaimableBalanceOperation {
    source_account: Option<MuxedAccount>,
    asset: Asset,
    amount: Stroops,
    claimants: Vec<Claimant>,
}

#[derive(Debug, Default)]
pub struct CreateClaimableBalanceOperationBuilder {
    source_account: Option<MuxedAccount>,
    asset: Option<Asset>,
    amount: Option<Stroops>,
    claimants: Vec<Claimant>,
}

impl CreateClaimableBalanceOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves a reference to the operation asset.
    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    /// Retrieves a mutable reference to the operation asset.
    pub fn asset_mut(&mut self) -> &mut Asset {
        &mut self.asset
    }

    /// Retrieves a reference to the operation amount.
    pub fn amount(&self) -> &Stroops {
        &self.amount
    }

    /// Retrieves a mutable reference to the operation amount.
    pub fn amount_mut(&mut self) -> &mut Stroops {
        &mut self.amount
    }

    /// Retrieves a reference to the operation claimants.
    pub fn claimants(&self) -> &Vec<Claimant> {
        &self.claimants
    }

    /// Retrieves a mutable reference to the operation claimants.
    pub fn claimants_mut(&mut self) -> &mut Vec<Claimant> {
        &mut self.claimants
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let asset = self.asset.to_xdr()?;
        let amount = self.amount.to_xdr_int64()?;
        let claimants_res: Result<Vec<xdr::Claimant>> =
            self.claimants.iter().map(|a| a.to_xdr()).collect();
        let claimants = claimants_res?;
        let inner = xdr::CreateClaimableBalanceOp {
            asset,
            amount,
            claimants: claimants.try_into().map_err(|_| Error::XdrError)?,
        };
        Ok(xdr::OperationBody::CreateClaimableBalance(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::CreateClaimableBalanceOp,
    ) -> Result<CreateClaimableBalanceOperation> {
        let asset = Asset::from_xdr(&x.asset)?;
        let amount = Stroops::from_xdr_int64(x.amount)?;
        let claimants_res: Result<Vec<Claimant>> =
            x.claimants.iter().map(Claimant::from_xdr).collect();
        let claimants = claimants_res?;
        Ok(CreateClaimableBalanceOperation {
            source_account,
            asset,
            amount,
            claimants,
        })
    }
}

impl CreateClaimableBalanceOperationBuilder {
    pub fn new() -> CreateClaimableBalanceOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> CreateClaimableBalanceOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_asset(mut self, asset: Asset) -> CreateClaimableBalanceOperationBuilder {
        self.asset = Some(asset);
        self
    }

    pub fn with_amount<A>(mut self, amount: A) -> Result<CreateClaimableBalanceOperationBuilder>
    where
        A: TryInto<Stroops>,
    {
        self.amount = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn with_claimants(
        mut self,
        claimants: Vec<Claimant>,
    ) -> CreateClaimableBalanceOperationBuilder {
        self.claimants = claimants;
        self
    }

    pub fn add_claimant(mut self, claimant: Claimant) -> CreateClaimableBalanceOperationBuilder {
        self.claimants.push(claimant);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let asset = self.asset.ok_or_else(|| {
            Error::InvalidOperation("missing create claimable balance asset".to_string())
        })?;

        let amount = self.amount.ok_or_else(|| {
            Error::InvalidOperation("missing create claimable balance amount".to_string())
        })?;

        if self.claimants.is_empty() {
            return Err(Error::InvalidOperation(
                "missing create claimable balance claimants".to_string(),
            ));
        }

        Ok(Operation::CreateClaimableBalance(
            CreateClaimableBalanceOperation {
                source_account: self.source_account,
                asset,
                amount,
                claimants: self.claimants,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::Amount;
    use crate::asset::Asset;
    use crate::claim::{ClaimPredicate, Claimant};

    use crate::network::Network;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use chrono::Duration;
    use std::str::FromStr;

    #[test]
    fn test_create_claimable_balance() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let amount = Amount::from_str("12.0333").unwrap();
        let asset = Asset::new_credit("ABCD", kp2.public_key()).unwrap();

        let predicate =
            ClaimPredicate::new_not(ClaimPredicate::new_before_relative_time(Duration::days(7)));

        let claimant = Claimant::new(kp1.public_key(), predicate);

        let op = Operation::new_create_claimable_balance()
            .with_asset(asset)
            .with_amount(amount)
            .unwrap()
            .add_claimant(claimant)
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAOAAAAAUFCQ0QAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAABywiyAAAAAEAAAAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAwAAAAEAAAAFAAAAAAAJOoAAAAAAAAAAAeoucsUAAABAUA3iWSLubKZc6r4CL2s9WTr/xMS5zuWgzxvm2hBs9use/2ejCagSPlRBeRCe3Ky4R+tKMk8Qpa2LATvgUQS2BQ==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_create_claimable_balance_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let amount = Amount::from_str("12.0333").unwrap();
        let asset = Asset::new_credit("ABCD", kp2.public_key()).unwrap();

        let predicate =
            ClaimPredicate::new_not(ClaimPredicate::new_before_relative_time(Duration::days(7)));

        let claimant = Claimant::new(kp1.public_key(), predicate);

        let op = Operation::new_create_claimable_balance()
            .with_source_account(kp.public_key())
            .with_asset(asset)
            .with_amount(amount)
            .unwrap()
            .add_claimant(claimant)
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAA4Nxt4XJcrGZRYrUvrOc1sooiQ+QdEk1suS1wo+oucsUAAAAOAAAAAUFCQ0QAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAABywiyAAAAAEAAAAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAwAAAAEAAAAFAAAAAAAJOoAAAAAAAAAAAeoucsUAAABAcaaQuqZMwpwVMS9814lZPhjt43B3xwlGNfeyx2wU2EJSDJ0h0d2a7dxngMzq4/abNVCjBKspCU7XroelAhSNCw==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
