use crate::asset::Asset;
use crate::error::{Error, Result};
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidityPoolId(Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidityPoolConstantFeeParameters {
    assets: (Asset, Asset),
    fee: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidityPoolParameters {
    ConstantFee(LiquidityPoolConstantFeeParameters),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidityPoolConstantFee {
    assets: (Asset, Asset),
    fee: i32,
    reserves: (i64, i64),
    total_pool_shares: i64,
    pool_shares_trust_line_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidityPool {
    ConstantFee(LiquidityPoolConstantFee),
}

impl LiquidityPoolId {
    /// Returns a new liquidity pool id, or Error if the hash length is not 32 bytes.
    pub fn new(hash: Vec<u8>) -> Result<Self> {
        if hash.len() != 32 {
            return Err(Error::InvalidLiquidityPoolIdLength);
        }

        Ok(Self(hash))
    }

    /// Retrieves the liquidity pool id bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> xdr::PoolId {
        let hash = xdr::Hash::new(self.0.clone());
        xdr::PoolId::new(hash)
    }

    /// Creates from the xdr object.
    pub fn from_xdr(x: &xdr::PoolId) -> Result<Self> {
        Ok(Self(x.value.value.clone()))
    }
}

impl LiquidityPoolConstantFeeParameters {
    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::LiquidityPoolConstantProductParameters> {
        Ok(xdr::LiquidityPoolConstantProductParameters {
            asset_a: self.assets.0.to_xdr()?,
            asset_b: self.assets.1.to_xdr()?,
            fee: xdr::Int32::new(self.fee),
        })
    }

    pub fn from_xdr(x: &xdr::LiquidityPoolConstantProductParameters) -> Result<Self> {
        let asset_a = Asset::from_xdr(&x.asset_a)?;
        let asset_b = Asset::from_xdr(&x.asset_b)?;
        let fee = x.fee.value;
        Ok(Self {
            assets: (asset_a, asset_b),
            fee,
        })
    }
}

impl LiquidityPoolParameters {
    pub fn to_xdr(&self) -> Result<xdr::LiquidityPoolParameters> {
        match self {
            LiquidityPoolParameters::ConstantFee(ref params) => {
                let params_xdr = params.to_xdr()?;
                Ok(xdr::LiquidityPoolParameters::LiquidityPoolConstantProduct(
                    params_xdr,
                ))
            }
        }
    }

    pub fn from_xdr(x: &xdr::LiquidityPoolParameters) -> Result<Self> {
        match *x {
            xdr::LiquidityPoolParameters::LiquidityPoolConstantProduct(ref constant_params) => {
                let params = LiquidityPoolConstantFeeParameters::from_xdr(constant_params)?;
                Ok(Self::ConstantFee(params))
            }
        }
    }
}
