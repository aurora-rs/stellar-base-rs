use crate::amount::Stroops;
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPaymentStrictReceiveOperation {
    source_account: Option<MuxedAccount>,
    destination: MuxedAccount,
    send_asset: Asset,
    send_max: Stroops,
    destination_asset: Asset,
    destination_amount: Stroops,
    path: Vec<Asset>,
}

#[derive(Debug, Default)]
pub struct PathPaymentStrictReceiveOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<MuxedAccount>,
    send_asset: Option<Asset>,
    send_max: Option<Stroops>,
    destination_asset: Option<Asset>,
    destination_amount: Option<Stroops>,
    path: Vec<Asset>,
}

impl PathPaymentStrictReceiveOperation {
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

    /// Retrieves the operatino send asset.
    pub fn send_asset(&self) -> &Asset {
        &self.send_asset
    }

    /// Retrieves a mutable reference to the operatino send asset.
    pub fn send_asset_mut(&mut self) -> &mut Asset {
        &mut self.send_asset
    }

    /// Retrieves the operation send max.
    pub fn send_max(&self) -> &Stroops {
        &self.send_max
    }

    /// Retrieves a mutable reference to the operation send max.
    pub fn send_max_mut(&mut self) -> &mut Stroops {
        &mut self.send_max
    }

    /// Retrieves the operation destination asset.
    pub fn destination_asset(&self) -> &Asset {
        &self.destination_asset
    }

    /// Retrieves a mutable reference to the operation destination asset.
    pub fn destination_asset_mut(&mut self) -> &mut Asset {
        &mut self.destination_asset
    }

    /// Retrieves the operation destination amount.
    pub fn destination_amount(&self) -> &Stroops {
        &self.destination_amount
    }

    /// Retrieves a mutable reference to the operation destination amount.
    pub fn destination_amount_mut(&mut self) -> &mut Stroops {
        &mut self.destination_amount
    }

    /// Retrieves the operation path.
    pub fn path(&self) -> &Vec<Asset> {
        &self.path
    }

    /// Retrieves a mutable reference to the operation path.
    pub fn path_mut(&mut self) -> &mut Vec<Asset> {
        &mut self.path
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let destination = self.destination.to_xdr()?;
        let send_asset = self.send_asset.to_xdr()?;
        let send_max = self.send_max.to_xdr_int64()?;
        let dest_asset = self.destination_asset.to_xdr()?;
        let dest_amount = self.destination_amount.to_xdr_int64()?;
        let path_res: Result<Vec<xdr::Asset>> = self.path.iter().map(|a| a.to_xdr()).collect();
        let path = path_res?;
        let inner = xdr::PathPaymentStrictReceiveOp {
            destination,
            send_asset,
            send_max,
            dest_asset,
            dest_amount,
            path,
        };
        Ok(xdr::OperationBody::PathPaymentStrictReceive(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::PathPaymentStrictReceiveOp,
    ) -> Result<PathPaymentStrictReceiveOperation> {
        let destination = MuxedAccount::from_xdr(&x.destination)?;
        let send_asset = Asset::from_xdr(&x.send_asset)?;
        let send_max = Stroops::from_xdr_int64(&x.send_max)?;
        let destination_asset = Asset::from_xdr(&x.dest_asset)?;
        let destination_amount = Stroops::from_xdr_int64(&x.dest_amount)?;
        let path_res: Result<Vec<Asset>> = x.path.iter().map(Asset::from_xdr).collect();
        let path = path_res?;
        Ok(PathPaymentStrictReceiveOperation {
            source_account,
            destination,
            send_asset,
            send_max,
            destination_asset,
            destination_amount,
            path,
        })
    }
}

impl PathPaymentStrictReceiveOperationBuilder {
    pub fn new() -> PathPaymentStrictReceiveOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> PathPaymentStrictReceiveOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_destination<A>(mut self, destination: A) -> PathPaymentStrictReceiveOperationBuilder
    where
        A: Into<MuxedAccount>,
    {
        self.destination = Some(destination.into());
        self
    }

    pub fn with_send_asset(
        mut self,
        send_asset: Asset,
    ) -> PathPaymentStrictReceiveOperationBuilder {
        self.send_asset = Some(send_asset);
        self
    }

    pub fn with_send_max<A>(
        mut self,
        send_max: A,
    ) -> Result<PathPaymentStrictReceiveOperationBuilder>
    where
        A: TryInto<Stroops>,
    {
        self.send_max = Some(
            send_max
                .try_into()
                .map_err(|_| Error::InvalidStroopsAmount)?,
        );
        Ok(self)
    }

    pub fn with_destination_asset(
        mut self,
        dest_asset: Asset,
    ) -> PathPaymentStrictReceiveOperationBuilder {
        self.destination_asset = Some(dest_asset);
        self
    }

    pub fn with_destination_amount<A>(
        mut self,
        dest_amount: A,
    ) -> Result<PathPaymentStrictReceiveOperationBuilder>
    where
        A: TryInto<Stroops>,
    {
        self.destination_amount = Some(
            dest_amount
                .try_into()
                .map_err(|_| Error::InvalidStroopsAmount)?,
        );
        Ok(self)
    }

    pub fn add_asset(mut self, asset: Asset) -> PathPaymentStrictReceiveOperationBuilder {
        self.path.push(asset);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let destination = self
            .destination
            .ok_or_else(|| Error::InvalidOperation("missing payment destination".to_string()))?;

        let send_asset = self.send_asset.ok_or_else(|| {
            Error::InvalidOperation("missing path payment strict send send asset".to_string())
        })?;

        let send_max = self.send_max.ok_or_else(|| {
            Error::InvalidOperation("missing path payment strict send send max".to_string())
        })?;

        let destination_asset = self.destination_asset.ok_or_else(|| {
            Error::InvalidOperation(
                "missing path payment strict send destination asset".to_string(),
            )
        })?;

        let destination_amount = self.destination_amount.ok_or_else(|| {
            Error::InvalidOperation(
                "missing path payment strict send destination amount".to_string(),
            )
        })?;

        if self.path.len() > 5 {
            return Err(Error::InvalidOperation(
                "path payment strict send path too long".to_string(),
            ));
        }

        Ok(Operation::PathPaymentStrictReceive(
            PathPaymentStrictReceiveOperation {
                source_account: self.source_account,
                destination,
                send_asset,
                send_max,
                destination_asset,
                destination_amount,
                path: self.path,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::Amount;
    use crate::asset::Asset;

    use crate::network::Network;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use std::str::FromStr;

    #[test]
    fn test_path_payment_strict_receive() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();
        let dest = kp1.public_key();

        let dest_amount = Amount::from_str("12.301").unwrap();
        let send_max = Amount::from_str("0.333").unwrap();

        let abcd = Asset::new_credit("ABCD", kp2.public_key()).unwrap();
        let dest_asset = Asset::new_credit("DESTASSET", kp2.public_key()).unwrap();

        let op = Operation::new_path_payment_strict_receive()
            .with_destination(dest)
            .with_send_asset(Asset::new_native())
            .with_send_max(send_max)
            .unwrap()
            .with_destination_asset(dest_asset)
            .with_destination_amount(dest_amount)
            .unwrap()
            .add_asset(abcd)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAACAAAAAAAAAAAAMs/QAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAkRFU1RBU1NFVAAAAAAAAAB+Ecs01jX14asC1KAsPdWlpGbYCM2PEgFZCD3NLhVZmAAAAAAHVPvQAAAAAQAAAAFBQkNEAAAAAH4RyzTWNfXhqwLUoCw91aWkZtgIzY8SAVkIPc0uFVmYAAAAAAAAAAHqLnLFAAAAQLZISKYSR3RXr9Hvxw1tr9P1B4fst/sDuQMGapBvSpLYU6DpDSOFM/vVEuB94HXWI79fSJmfyEl+gR6Zh+o0Yw4=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_path_payment_strict_receive_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();
        let dest = kp1.public_key();

        let dest_amount = Amount::from_str("12.301").unwrap();
        let send_max = Amount::from_str("0.333").unwrap();

        let abcd = Asset::new_credit("ABCD", kp2.public_key()).unwrap();
        let dest_asset = Asset::new_credit("DESTASSET", kp2.public_key()).unwrap();

        let op = Operation::new_path_payment_strict_receive()
            .with_source_account(kp1.public_key())
            .with_destination(dest)
            .with_send_asset(Asset::new_native())
            .with_send_max(send_max)
            .unwrap()
            .with_destination_asset(dest_asset)
            .with_destination_amount(dest_amount)
            .unwrap()
            .add_asset(abcd)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAACAAAAAAAAAAAAMs/QAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAkRFU1RBU1NFVAAAAAAAAAB+Ecs01jX14asC1KAsPdWlpGbYCM2PEgFZCD3NLhVZmAAAAAAHVPvQAAAAAQAAAAFBQkNEAAAAAH4RyzTWNfXhqwLUoCw91aWkZtgIzY8SAVkIPc0uFVmYAAAAAAAAAAHqLnLFAAAAQJgKu/fRcqT/SwSy1ejitxV6hGH/CZZtv+Qoe1usuSK2kN0UYz6YQOy0aqMwP1iJrIV5DbiDRymKEdEZAo9a5Q4=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
