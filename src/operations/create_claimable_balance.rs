use crate::amount::Stroops;
use crate::asset::Asset;
use crate::claim::Claimant;
use crate::crypto::{MuxedAccount, PublicKey};
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
            claimants,
        };
        Ok(xdr::OperationBody::CreateClaimableBalance(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::CreateClaimableBalanceOp,
    ) -> Result<CreateClaimableBalanceOperation> {
        let asset = Asset::from_xdr(&x.asset)?;
        let amount = Stroops::from_xdr_int64(&x.amount)?;
        let claimants_res: Result<Vec<Claimant>> =
            x.claimants.iter().map(|c| Claimant::from_xdr(&c)).collect();
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