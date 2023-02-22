use crate::amount::Stroops;
use crate::asset::{xdr_code_to_string, CreditAsset};
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::liquidity_pool::LiquidityPoolParameters;
use crate::operations::Operation;
use crate::{xdr, PublicKey};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeTrustOperation {
    source_account: Option<MuxedAccount>,
    asset: ChangeTrustAsset,
    limit: Option<Stroops>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeTrustAsset {
    Native,
    Credit(CreditAsset),
    PoolShare(LiquidityPoolParameters),
}

#[derive(Debug, Default)]
pub struct ChangeTrustOperationBuilder {
    source_account: Option<MuxedAccount>,
    asset: Option<ChangeTrustAsset>,
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
    pub fn asset(&self) -> &ChangeTrustAsset {
        &self.asset
    }

    /// Retrieves a mutable reference the operation asset.
    pub fn asset_mut(&mut self) -> &mut ChangeTrustAsset {
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

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ChangeTrustOp,
    ) -> Result<ChangeTrustOperation> {
        let asset = ChangeTrustAsset::from_xdr(&x.line)?;
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

impl ChangeTrustAsset {
    pub fn new_native() -> Result<Self> {
        Ok(Self::Native)
    }

    pub fn new_credit<S>(code: S, issuer: PublicKey) -> Result<Self>
    where
        S: Into<String>,
    {
        let code = code.into();
        let inner = CreditAsset::new(code, issuer)?;
        Ok(Self::Credit(inner))
    }

    pub fn new_pool_share(pool_params: LiquidityPoolParameters) -> Result<Self> {
        Ok(Self::PoolShare(pool_params))
    }

    pub fn to_xdr(&self) -> Result<xdr::ChangeTrustAsset> {
        match self {
            Self::Native => Ok(xdr::ChangeTrustAsset::AssetTypeNative(())),
            Self::Credit(ref credit) => match credit {
                CreditAsset::AlphaNum4 { code, issuer } => {
                    let code_len = code.len();
                    let mut code_bytes = vec![0; 4];
                    code_bytes[..code_len].copy_from_slice(code.as_bytes());
                    let asset_code = xdr::AssetCode4::new(code_bytes);
                    let issuer = issuer.to_xdr_account_id()?;
                    let asset_alphanum4 = xdr::AlphaNum4 { asset_code, issuer };
                    Ok(xdr::ChangeTrustAsset::AssetTypeCreditAlphanum4(
                        asset_alphanum4,
                    ))
                }
                CreditAsset::AlphaNum12 { code, issuer } => {
                    let code_len = code.len();
                    let mut code_bytes = vec![0; 12];
                    code_bytes[..code_len].copy_from_slice(code.as_bytes());
                    let asset_code = xdr::AssetCode12::new(code_bytes);
                    let issuer = issuer.to_xdr_account_id()?;
                    let asset_alphanum12 = xdr::AlphaNum12 { asset_code, issuer };
                    Ok(xdr::ChangeTrustAsset::AssetTypeCreditAlphanum12(
                        asset_alphanum12,
                    ))
                }
            },
            Self::PoolShare(ref pool_params) => {
                let inner_xdr = pool_params.to_xdr()?;
                Ok(xdr::ChangeTrustAsset::AssetTypePoolShare(inner_xdr))
            }
        }
    }

    pub fn from_xdr(x: &xdr::ChangeTrustAsset) -> Result<Self> {
        match *x {
            xdr::ChangeTrustAsset::AssetTypeNative(()) => Self::new_native(),
            xdr::ChangeTrustAsset::AssetTypeCreditAlphanum4(ref credit) => {
                let issuer = PublicKey::from_xdr_account_id(&credit.issuer)?;
                let code = xdr_code_to_string(&credit.asset_code.value);
                Self::new_credit(code, issuer)
            }
            xdr::ChangeTrustAsset::AssetTypeCreditAlphanum12(ref credit) => {
                let issuer = PublicKey::from_xdr_account_id(&credit.issuer)?;
                let code = xdr_code_to_string(&credit.asset_code.value);
                Self::new_credit(code, issuer)
            }
            xdr::ChangeTrustAsset::AssetTypePoolShare(ref pool_params_xdr) => {
                let pool_params = LiquidityPoolParameters::from_xdr(pool_params_xdr)?;
                Self::new_pool_share(pool_params)
            }
        }
    }
}

impl ChangeTrustOperationBuilder {
    pub fn new() -> ChangeTrustOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> ChangeTrustOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_asset(mut self, asset: ChangeTrustAsset) -> ChangeTrustOperationBuilder {
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
            if limit.to_i64() < 0 {
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
    use crate::amount::Stroops;

    use crate::network::Network;
    use crate::operations::change_trust::ChangeTrustAsset;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_change_trust() {
        let kp = keypair0();
        let kp1 = keypair1();

        let asset = ChangeTrustAsset::new_credit("FOOBAR", kp1.public_key().clone()).unwrap();

        let op = Operation::new_change_trust()
            .with_asset(asset)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp.as_ref(), &Network::new_test()).unwrap();
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

        let asset = ChangeTrustAsset::new_credit("FOOBAR", kp1.public_key().clone()).unwrap();

        let op = Operation::new_change_trust()
            .with_asset(asset)
            .with_limit(Some(Stroops::max()))
            .unwrap()
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp.as_ref(), &Network::new_test()).unwrap();
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

        let asset = ChangeTrustAsset::new_credit("FOOBAR", kp1.public_key().clone()).unwrap();

        let op = Operation::new_change_trust()
            .with_source_account(kp2.public_key().clone())
            .with_asset(asset)
            .with_limit(Some(Stroops::max()))
            .unwrap()
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAGAAAAAkZPT0JBUgAAAAAAAAAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfn//////////AAAAAAAAAAHqLnLFAAAAQOLqKZ6HQ5VeJvRyWk9FBA5z6UVoxKPXAODzj7c0sOr6tGYzbAtHW1ahOPdIOI03J4lGjM21ROvgi3ClSfZVUAc=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
