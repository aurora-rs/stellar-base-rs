use crate::amount::Stroops;
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeTrustOperation {
    source_account: Option<MuxedAccount>,
    asset: Asset,
    limit: Option<Stroops>,
}

#[derive(Debug)]
pub struct ChangeTrustOperationBuilder {
    source_account: Option<MuxedAccount>,
    asset: Option<Asset>,
    limit: Option<Stroops>,
}

impl ChangeTrustOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation asset.
    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    /// Retrieves a mutable reference the operation asset.
    pub fn asset_mut(&mut self) -> &mut Asset {
        &mut self.asset
    }

    /// Retrieves the operation limit.
    pub fn limit(&self) -> &Option<Stroops> {
        &self.limit
    }

    /// Retrieves a mutable reference to the operation limit.
    pub fn limit_mut(&mut self) -> &mut Option<Stroops> {
        &mut self.limit
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let line = self.asset.to_xdr()?;
        let limit = match &self.limit {
            None => xdr::Int64::new(0),
            Some(limit) => limit.to_xdr_int64()?,
        };
        let inner = xdr::ChangeTrustOp { line, limit };
        Ok(xdr::OperationBody::ChangeTrust(inner))
    }

    /// Creates from the xdr operatino body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ChangeTrustOp,
    ) -> Result<ChangeTrustOperation> {
        let asset = Asset::from_xdr(&x.line)?;
        // Don't check if limit is positive because the library sure
        // has no control over the xdr.
        let limit = match &x.limit.value {
            0 => None,
            n => Some(Stroops::new(*n)),
        };
        Ok(ChangeTrustOperation {
            source_account,
            asset,
            limit,
        })
    }
}

impl ChangeTrustOperationBuilder {
    pub fn new() -> ChangeTrustOperationBuilder {
        ChangeTrustOperationBuilder {
            source_account: None,
            asset: None,
            limit: None,
        }
    }

    pub fn with_source_account<S>(mut self, source: S) -> ChangeTrustOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_asset(mut self, asset: Asset) -> ChangeTrustOperationBuilder {
        self.asset = Some(asset);
        self
    }

    pub fn with_limit<A: TryInto<Stroops>>(
        mut self,
        limit: Option<A>,
    ) -> Result<ChangeTrustOperationBuilder> {
        self.limit = limit
            .map(|l| l.try_into())
            .transpose()
            .map_err(|_| Error::InvalidStroopsAmount)?;
        Ok(self)
    }

    pub fn build(self) -> Result<Operation> {
        let asset = self
            .asset
            .ok_or_else(|| Error::InvalidOperation("missing change trust asset".to_string()))?;

        if let Some(limit) = &self.limit {
            if limit.to_i64() <= 0 {
                return Err(Error::InvalidOperation(
                    "change trust limit must be positive".to_string(),
                ));
            }
        }

        Ok(Operation::ChangeTrust(ChangeTrustOperation {
            source_account: self.source_account,
            asset,
            limit: self.limit,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::account::{AccountFlags, DataValue, TrustLineFlags};
    use crate::amount::{Amount, Price, Stroops};
    use crate::asset::{Asset, CreditAssetType};
    use crate::crypto::KeyPair;
    use crate::memo::Memo;
    use crate::network::Network;
    use crate::operations::Operation;
    use crate::time_bounds::TimeBounds;
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
    fn test_change_trust() {
        let kp = keypair0();
        let kp1 = keypair1();

        let asset = Asset::new_credit("FOOBAR", kp1.public_key().clone()).unwrap();

        let op = Operation::new_change_trust()
            .with_asset(asset)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test());
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAGAAAAAkZPT0JBUgAAAAAAAAAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfgAAAAAAAAAAAAAAAAAAAAHqLnLFAAAAQItMBQHM0xqmxrCWDEUHHLm8RZZAamXvBVCFovprlCnzAwGOCgo/HrZlKhZL5LMJtoUAgTtb9eQL+Ur7H8zKDAk=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_change_trust_with_limit() {
        let kp = keypair0();
        let kp1 = keypair1();

        let asset = Asset::new_credit("FOOBAR", kp1.public_key().clone()).unwrap();

        let op = Operation::new_change_trust()
            .with_asset(asset)
            .with_limit(Some(Stroops::max()))
            .unwrap()
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test());
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAGAAAAAkZPT0JBUgAAAAAAAAAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfn//////////AAAAAAAAAAHqLnLFAAAAQBGXSIMx1RSjmS7XD9DluNCn6TolNnB9sdmvBSlWeaizwgfud6hD8BZSfqBHdTNm4DgmloojC9fIVRtVFEHhpAE=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_change_trust_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let asset = Asset::new_credit("FOOBAR", kp1.public_key().clone()).unwrap();

        let op = Operation::new_change_trust()
            .with_source_account(kp2.public_key().clone())
            .with_asset(asset)
            .with_limit(Some(Stroops::max()))
            .unwrap()
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test());
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAGAAAAAkZPT0JBUgAAAAAAAAAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfn//////////AAAAAAAAAAHqLnLFAAAAQOLqKZ6HQ5VeJvRyWk9FBA5z6UVoxKPXAODzj7c0sOr6tGYzbAtHW1ahOPdIOI03J4lGjM21ROvgi3ClSfZVUAc=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
