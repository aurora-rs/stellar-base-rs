use crate::account::TrustLineFlags;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::{xdr, Asset, Operation, PublicKey};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetTrustLineFlagsOperation {
    source_account: Option<MuxedAccount>,
    trustor: PublicKey,
    asset: Asset,
    clear_flags: TrustLineFlags,
    set_flags: TrustLineFlags,
}

#[derive(Debug, Default)]
pub struct SetTrustLineFlagsOperationBuilder {
    source_account: Option<MuxedAccount>,
    trustor: Option<PublicKey>,
    asset: Option<Asset>,
    clear_flags: Option<TrustLineFlags>,
    set_flags: Option<TrustLineFlags>,
}

impl SetTrustLineFlagsOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    pub fn trustor(&self) -> &PublicKey {
        &self.trustor
    }

    pub fn trustor_mut(&mut self) -> &mut PublicKey {
        &mut self.trustor
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    pub fn asset_mut(&mut self) -> &mut Asset {
        &mut self.asset
    }

    pub fn clear_flags(&self) -> &TrustLineFlags {
        &self.clear_flags
    }

    pub fn clear_flags_mut(&mut self) -> &mut TrustLineFlags {
        &mut self.clear_flags
    }

    pub fn set_flags(&self) -> &TrustLineFlags {
        &self.set_flags
    }

    pub fn set_flags_mut(&mut self) -> &mut TrustLineFlags {
        &mut self.set_flags
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inner = xdr::SetTrustLineFlagsOp {
            trustor: self.trustor.to_xdr_account_id()?,
            asset: self.asset.to_xdr()?,
            clear_flags: self.clear_flags.bits(),
            set_flags: self.set_flags.bits(),
        };
        Ok(xdr::OperationBody::SetTrustLineFlags(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::SetTrustLineFlagsOp,
    ) -> Result<Self> {
        Ok(Self {
            source_account,
            trustor: PublicKey::from_xdr_account_id(&x.trustor)?,
            asset: Asset::from_xdr(&x.asset)?,
            clear_flags: TrustLineFlags::from_bits(x.clear_flags)
                .ok_or(Error::InvalidTrustLineFlags)?,
            set_flags: TrustLineFlags::from_bits(x.set_flags)
                .ok_or(Error::InvalidTrustLineFlags)?,
        })
    }
}

impl SetTrustLineFlagsOperationBuilder {
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

    pub fn with_trustor(mut self, trustor: PublicKey) -> Self {
        self.trustor = Some(trustor);
        self
    }

    pub fn with_asset(mut self, asset: Asset) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn with_clear_flags(mut self, clear_flags: TrustLineFlags) -> Self {
        self.clear_flags = Some(clear_flags);
        self
    }

    pub fn with_set_flags(mut self, set_flags: TrustLineFlags) -> Self {
        self.set_flags = Some(set_flags);
        self
    }

    pub fn build(self) -> Result<Operation> {
        if self.set_flags.is_none() && self.clear_flags.is_none() {
            return Err(Error::InvalidOperation(
                "either set or clear flags are needed for set trustline flags operation"
                    .to_string(),
            ));
        }

        let trustor = self.trustor.ok_or_else(|| {
            Error::InvalidOperation("missing trustor for set trustline flags operation".to_string())
        })?;

        let asset = self.asset.ok_or_else(|| {
            Error::InvalidOperation("missing asset for set trustline flags operation".to_string())
        })?;

        let clear_flags = self.clear_flags.unwrap_or_else(TrustLineFlags::empty);
        let set_flags = self.set_flags.unwrap_or_else(TrustLineFlags::empty);

        Ok(Operation::SetTrustLineFlags(SetTrustLineFlagsOperation {
            source_account: self.source_account,
            trustor,
            asset,
            clear_flags,
            set_flags,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::account::TrustLineFlags;
    use crate::operations::tests::*;
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use crate::{Asset, Operation, PublicKey};

    fn asset0() -> Asset {
        let issuer =
            PublicKey::from_account_id("GBO7YJU3JMAGYVXDYA3IYZ2GZS2JKPRZIHMUY5MYKKJPJUBYWAGDWXQG")
                .unwrap();
        Asset::new_credit("TEST", issuer).unwrap()
    }

    #[test]
    fn test_set_trustline_flags() {
        let trustor = keypair1().public_key();
        let asset = asset0();
        let clear_flags = TrustLineFlags::AUTHORIZED_TO_MAINTAIN_LIABILITIES;
        let set_flags = TrustLineFlags::TRUSTLINE_CLAWBACK_ENABLED;

        let op = Operation::new_set_trustline_flags()
            .with_trustor(trustor)
            .with_asset(asset)
            .with_clear_flags(clear_flags)
            .with_set_flags(set_flags)
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let expected = "AAAAAAAAABUAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAABVEVTVAAAAABd/CabSwBsVuPANoxnRsy0lT45QdlMdZhSkvTQOLAMOwAAAAIAAAAE";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, back);
    }

    #[test]
    fn test_set_trustline_flags_with_source_account() {
        let source_account = keypair0().public_key();
        let trustor = keypair1().public_key();
        let asset = asset0();
        let clear_flags = TrustLineFlags::AUTHORIZED_TO_MAINTAIN_LIABILITIES;
        let set_flags = TrustLineFlags::TRUSTLINE_CLAWBACK_ENABLED;

        let op = Operation::new_set_trustline_flags()
            .with_source_account(source_account)
            .with_trustor(trustor)
            .with_asset(asset)
            .with_clear_flags(clear_flags)
            .with_set_flags(set_flags)
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let expected = "AAAAAQAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAABUAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAABVEVTVAAAAABd/CabSwBsVuPANoxnRsy0lT45QdlMdZhSkvTQOLAMOwAAAAIAAAAE";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, back);
    }
}
