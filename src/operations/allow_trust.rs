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
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn trustor(&self) -> &PublicKey {
        &self.trustor
    }

    pub fn asset(&self) -> &CreditAssetType {
        &self.asset
    }

    pub fn authorize_flags(&self) -> &TrustLineFlags {
        &self.authorize
    }

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

    pub fn with_source_account(mut self, source: MuxedAccount) -> AllowTrustOperationBuilder {
        self.source_account = Some(source);
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
