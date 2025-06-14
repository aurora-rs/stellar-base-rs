use crate::amount::Stroops;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::{xdr, Asset, Operation};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClawbackOperation {
    source_account: Option<MuxedAccount>,
    asset: Asset,
    from: MuxedAccount,
    amount: Stroops,
}

#[derive(Debug, Default)]
pub struct ClawbackOperationBuilder {
    source_account: Option<MuxedAccount>,
    asset: Option<Asset>,
    from: Option<MuxedAccount>,
    amount: Option<Stroops>,
}

impl ClawbackOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    pub fn asset_mut(&mut self) -> &mut Asset {
        &mut self.asset
    }

    pub fn from(&self) -> &MuxedAccount {
        &self.from
    }

    pub fn from_mut(&mut self) -> &mut MuxedAccount {
        &mut self.from
    }

    pub fn amount(&self) -> &Stroops {
        &self.amount
    }

    pub fn amount_mut(&mut self) -> &mut Stroops {
        &mut self.amount
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inner = xdr::ClawbackOp {
            asset: self.asset.to_xdr()?,
            from: self.from.to_xdr()?,
            amount: self.amount.to_xdr_int64()?,
        };
        Ok(xdr::OperationBody::Clawback(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ClawbackOp,
    ) -> Result<Self> {
        Ok(Self {
            source_account,
            asset: Asset::from_xdr(&x.asset)?,
            from: MuxedAccount::from_xdr(&x.from)?,
            amount: Stroops::from_xdr_int64(x.amount)?,
        })
    }
}

impl ClawbackOperationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> Self
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_asset(mut self, asset: Asset) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn with_from<S>(mut self, from: S) -> Self
    where
        S: Into<MuxedAccount>,
    {
        self.from = Some(from.into());
        self
    }

    pub fn with_amount<S>(mut self, amount: S) -> Result<Self>
    where
        S: TryInto<Stroops>,
    {
        self.amount = Some(amount.try_into().map_err(|_| Error::InvalidStroopsAmount)?);
        Ok(self)
    }

    pub fn build(self) -> Result<Operation> {
        let asset = self
            .asset
            .ok_or_else(|| Error::InvalidOperation("missing clawback asset".to_string()))?;

        let from = self
            .from
            .ok_or_else(|| Error::InvalidOperation("missing clawback from account".to_string()))?;

        let amount = self
            .amount
            .ok_or_else(|| Error::InvalidOperation("missing clawback amount".to_string()))?;

        Ok(Operation::Clawback(ClawbackOperation {
            source_account: self.source_account,
            asset,
            from,
            amount,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::Amount;
    use crate::operations::tests::*;
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use crate::{Asset, Operation, PublicKey};
    use std::str::FromStr;

    fn asset0() -> Asset {
        let issuer =
            PublicKey::from_account_id("GBO7YJU3JMAGYVXDYA3IYZ2GZS2JKPRZIHMUY5MYKKJPJUBYWAGDWXQG")
                .unwrap();
        Asset::new_credit("TEST", issuer).unwrap()
    }

    #[test]
    fn test_clawback() {
        let from = keypair1().public_key();
        let asset = asset0();
        let amount = Amount::from_str("17.6301").unwrap();

        let clawback_op = Operation::new_clawback()
            .with_from(from)
            .with_asset(asset)
            .with_amount(amount)
            .unwrap()
            .build()
            .unwrap();

        let encoded = clawback_op.xdr_base64().unwrap();
        let expected = "AAAAAAAAABMAAAABVEVTVAAAAABd/CabSwBsVuPANoxnRsy0lT45QdlMdZhSkvTQOLAMOwAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfgAAAAAKgiPI";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(clawback_op, back);
    }

    #[test]
    fn test_clawback_with_source_account() {
        let source_account = keypair0().public_key();
        let from = keypair1().public_key();
        let asset = asset0();
        let amount = Amount::from_str("17.6301").unwrap();

        let clawback_op = Operation::new_clawback()
            .with_source_account(source_account)
            .with_from(from)
            .with_asset(asset)
            .with_amount(amount)
            .unwrap()
            .build()
            .unwrap();

        let encoded = clawback_op.xdr_base64().unwrap();
        let expected = "AAAAAQAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAABMAAAABVEVTVAAAAABd/CabSwBsVuPANoxnRsy0lT45QdlMdZhSkvTQOLAMOwAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfgAAAAAKgiPI";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(clawback_op, back);
    }
}
