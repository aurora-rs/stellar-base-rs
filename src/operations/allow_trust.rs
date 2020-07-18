use crate::account::TrustLineFlags;
use crate::asset::{xdr_code_to_string, CreditAssetType};
use crate::crypto::{MuxedAccount, PublicKey};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowTrustOperation {
    source_account: Option<MuxedAccount>,
    trustor: PublicKey,
    asset: CreditAssetType,
    authorize: TrustLineFlags,
}

#[derive(Debug)]
pub struct AllowTrustOperationBuilder {
    source_account: Option<MuxedAccount>,
    trustor: Option<PublicKey>,
    asset: Option<CreditAssetType>,
    authorize: Option<TrustLineFlags>,
}

impl AllowTrustOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation trustor.
    pub fn trustor(&self) -> &PublicKey {
        &self.trustor
    }

    /// Retrieves a mutable reference to the operation trustor.
    pub fn trustor_mut(&mut self) -> &mut PublicKey {
        &mut self.trustor
    }

    /// Retrieves the operation asset.
    pub fn asset(&self) -> &CreditAssetType {
        &self.asset
    }

    /// Retrieves a mutable reference to the operation asset.
    pub fn asset_mut(&mut self) -> &mut CreditAssetType {
        &mut self.asset
    }

    /// Retrieves the operation authorize flags.
    pub fn authorize_flags(&self) -> &TrustLineFlags {
        &self.authorize
    }

    /// Retrieves a mutable reference to the operation authorize flags.
    pub fn authorize_flags_mut(&mut self) -> &mut TrustLineFlags {
        &mut self.authorize
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let trustor = self.trustor.to_xdr_account_id()?;
        let asset = match &self.asset {
            CreditAssetType::CreditAlphaNum4(code) => {
                let code_len = code.len();
                let mut code_bytes = Vec::with_capacity(4);
                code_bytes.resize(4, 0);
                code_bytes[..code_len].copy_from_slice(code.as_bytes());
                let asset_code = xdr::AssetCode4::new(code_bytes);
                xdr::AllowTrustOpAsset::AssetTypeCreditAlphanum4(asset_code)
            }
            CreditAssetType::CreditAlphaNum12(code) => {
                let code_len = code.len();
                let mut code_bytes = Vec::with_capacity(12);
                code_bytes.resize(12, 0);
                code_bytes[..code_len].copy_from_slice(code.as_bytes());
                let asset_code = xdr::AssetCode12::new(code_bytes);
                xdr::AllowTrustOpAsset::AssetTypeCreditAlphanum12(asset_code)
            }
        };
        let authorize = xdr::Uint32::new(self.authorize.bits());

        let inner = xdr::AllowTrustOp {
            trustor,
            asset,
            authorize,
        };
        Ok(xdr::OperationBody::AllowTrust(inner))
    }

    /// Creates from xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::AllowTrustOp,
    ) -> Result<AllowTrustOperation> {
        let trustor = PublicKey::from_xdr_account_id(&x.trustor)?;
        let asset = match &x.asset {
            xdr::AllowTrustOpAsset::AssetTypeCreditAlphanum4(code) => {
                let code = xdr_code_to_string(&code.value);
                CreditAssetType::CreditAlphaNum4(code)
            }
            xdr::AllowTrustOpAsset::AssetTypeCreditAlphanum12(code) => {
                let code = xdr_code_to_string(&code.value);
                CreditAssetType::CreditAlphaNum12(code)
            }
        };
        let authorize =
            TrustLineFlags::from_bits(x.authorize.value).ok_or(Error::InvalidTrustLineFlags)?;

        Ok(AllowTrustOperation {
            source_account,
            trustor,
            asset,
            authorize,
        })
    }
}

impl AllowTrustOperationBuilder {
    pub fn new() -> AllowTrustOperationBuilder {
        AllowTrustOperationBuilder {
            source_account: None,
            trustor: None,
            asset: None,
            authorize: None,
        }
    }

    pub fn with_source_account<S>(mut self, source: S) -> AllowTrustOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_trustor(mut self, trustor: PublicKey) -> AllowTrustOperationBuilder {
        self.trustor = Some(trustor);
        self
    }

    pub fn with_asset(mut self, asset: CreditAssetType) -> AllowTrustOperationBuilder {
        self.asset = Some(asset);
        self
    }

    pub fn with_authorize_flags(mut self, authorize: TrustLineFlags) -> AllowTrustOperationBuilder {
        self.authorize = Some(authorize);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let trustor = self
            .trustor
            .ok_or_else(|| Error::InvalidOperation("missing allow trust trustor".to_string()))?;

        let asset = self
            .asset
            .ok_or_else(|| Error::InvalidOperation("missing allow trust asset".to_string()))?;

        let authorize = self.authorize.ok_or_else(|| {
            Error::InvalidOperation("missing allow trust authorize flags".to_string())
        })?;

        Ok(Operation::AllowTrust(AllowTrustOperation {
            source_account: self.source_account,
            trustor,
            asset,
            authorize,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::account::TrustLineFlags;
    use crate::asset::CreditAssetType;
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
    fn test_allow_trust() {
        let kp = keypair0();
        let kp1 = keypair1();

        let op = Operation::new_allow_trust()
            .with_trustor(kp1.public_key().clone())
            .with_asset(CreditAssetType::CreditAlphaNum4("ABCD".to_string()))
            .with_authorize_flags(TrustLineFlags::AUTHORIZED)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAHAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAUFCQ0QAAAABAAAAAAAAAAHqLnLFAAAAQNhV5kJkZryrHEq8jgx9O76dchfHSkS99FTAcR6D2cjSoy6dbPuGsiPpTbwbMMV+lYTigEmv5vTVV+rWcLfr0Q0=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_allow_trust_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let op = Operation::new_allow_trust()
            .with_source_account(kp2.public_key().clone())
            .with_trustor(kp1.public_key().clone())
            .with_asset(CreditAssetType::CreditAlphaNum4("ABCD".to_string()))
            .with_authorize_flags(TrustLineFlags::AUTHORIZED)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAHAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAUFCQ0QAAAABAAAAAAAAAAHqLnLFAAAAQOGeHFq7smaQekF9Cu+duxVIGbMiGvT5CccilyMmB7WELOn85XuYIY6qfDKCnjgH47ga1Yve8qnA5hdD2A+iAgA=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
