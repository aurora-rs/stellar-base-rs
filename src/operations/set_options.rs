use crate::account::AccountFlags;
use crate::crypto::{MuxedAccount, PublicKey, Signer};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetOptionsOperation {
    source_account: Option<MuxedAccount>,
    inflation_destination: Option<PublicKey>,
    clear_flags: Option<AccountFlags>,
    set_flags: Option<AccountFlags>,
    master_weight: Option<u32>,
    low_threshold: Option<u32>,
    medium_threshold: Option<u32>,
    high_threshold: Option<u32>,
    home_domain: Option<String>,
    signer: Option<Signer>,
}

#[derive(Debug, Default)]
pub struct SetOptionsOperationBuilder {
    source_account: Option<MuxedAccount>,
    inflation_destination: Option<PublicKey>,
    clear_flags: Option<AccountFlags>,
    set_flags: Option<AccountFlags>,
    master_weight: Option<u32>,
    low_threshold: Option<u32>,
    medium_threshold: Option<u32>,
    high_threshold: Option<u32>,
    home_domain: Option<String>,
    signer: Option<Signer>,
}

impl SetOptionsOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation inflation destination.
    pub fn inflation_destination(&self) -> &Option<PublicKey> {
        &self.inflation_destination
    }

    /// Retrieves a mutable reference to the operation inflation destination.
    pub fn inflation_destination_mut(&mut self) -> &mut Option<PublicKey> {
        &mut self.inflation_destination
    }

    /// Retrieves the operation clear flags.
    pub fn clear_flags(&self) -> &Option<AccountFlags> {
        &self.clear_flags
    }

    /// Retrieves a mutable reference to the operation clear flags.
    pub fn clear_flags_mut(&mut self) -> &mut Option<AccountFlags> {
        &mut self.clear_flags
    }

    /// Retrieves the operation set flags.
    pub fn set_flags(&self) -> &Option<AccountFlags> {
        &self.set_flags
    }

    /// Retrieves a mutable reference to the operation set flags.
    pub fn set_flags_mut(&mut self) -> &mut Option<AccountFlags> {
        &mut self.set_flags
    }

    /// Retrieves the operation master weight.
    pub fn master_weight(&self) -> &Option<u32> {
        &self.master_weight
    }

    /// Retrieves a mutable reference to the operation master weight.
    pub fn master_weight_mut(&mut self) -> &mut Option<u32> {
        &mut self.master_weight
    }

    /// Retrieves the operation low threshold.
    pub fn low_threshold(&self) -> &Option<u32> {
        &self.low_threshold
    }

    /// Retrieves a mutable reference to the operation low threshold.
    pub fn low_threshold_mut(&mut self) -> &mut Option<u32> {
        &mut self.low_threshold
    }

    /// Retrieves the operation medium threshold.
    pub fn medium_threshold(&self) -> &Option<u32> {
        &self.medium_threshold
    }

    /// Retrieves a mutable reference to the operation medium threshold.
    pub fn medium_threshold_mut(&mut self) -> &mut Option<u32> {
        &mut self.medium_threshold
    }

    /// Retrieves the operation high threshold.
    pub fn high_threshold(&self) -> &Option<u32> {
        &self.high_threshold
    }

    /// Retrieves a mutable reference to the operation high threshold.
    pub fn high_threshold_mut(&mut self) -> &mut Option<u32> {
        &mut self.high_threshold
    }

    /// Retrieves the operation home domain.
    pub fn home_domain(&self) -> &Option<String> {
        &self.home_domain
    }

    /// Retrieves a mutable reference to the operation home domain.
    pub fn home_domain_mut(&mut self) -> &mut Option<String> {
        &mut self.home_domain
    }

    /// Retrieves the operation signer.
    pub fn signer(&self) -> &Option<Signer> {
        &self.signer
    }

    /// Retrieves a mutable reference the operation signer.
    pub fn signer_mut(&mut self) -> &mut Option<Signer> {
        &mut self.signer
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inflation_dest = self
            .inflation_destination
            .as_ref()
            .map(|d| d.to_xdr_account_id())
            .transpose()?;
        let clear_flags = self.clear_flags.map(|f| xdr::Uint32::new(f.bits()));
        let set_flags = self.set_flags.map(|f| xdr::Uint32::new(f.bits()));
        let master_weight = self.master_weight.map(xdr::Uint32::new);
        let low_threshold = self.low_threshold.map(xdr::Uint32::new);
        let med_threshold = self.medium_threshold.map(xdr::Uint32::new);
        let high_threshold = self.high_threshold.map(xdr::Uint32::new);
        let signer = self.signer.as_ref().map(|s| s.to_xdr()).transpose()?;

        if let Some(home_domain) = &self.home_domain {
            if home_domain.len() > 32 {
                return Err(Error::HomeDomainTooLong);
            }
        }

        let home_domain = self
            .home_domain
            .as_ref()
            .map(|h| xdr::String32::new(h.to_string()));

        let inner = xdr::SetOptionsOp {
            inflation_dest,
            clear_flags,
            set_flags,
            master_weight,
            low_threshold,
            med_threshold,
            high_threshold,
            home_domain,
            signer,
        };

        Ok(xdr::OperationBody::SetOptions(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::SetOptionsOp,
    ) -> Result<SetOptionsOperation> {
        let inflation_destination = x
            .inflation_dest
            .as_ref()
            .map(PublicKey::from_xdr_account_id)
            .transpose()?;
        let clear_flags = x
            .clear_flags
            .as_ref()
            .map(|f| AccountFlags::from_bits(f.value).ok_or(Error::InvalidAccountFlags))
            .transpose()?;
        let set_flags = x
            .set_flags
            .as_ref()
            .map(|f| AccountFlags::from_bits(f.value).ok_or(Error::InvalidAccountFlags))
            .transpose()?;
        let master_weight = x.master_weight.as_ref().map(|w| w.value);
        let low_threshold = x.low_threshold.as_ref().map(|w| w.value);
        let medium_threshold = x.med_threshold.as_ref().map(|w| w.value);
        let high_threshold = x.high_threshold.as_ref().map(|w| w.value);
        let home_domain = x.home_domain.as_ref().map(|h| h.value.clone());
        let signer = x.signer.as_ref().map(Signer::from_xdr).transpose()?;
        Ok(SetOptionsOperation {
            source_account,
            inflation_destination,
            clear_flags,
            set_flags,
            master_weight,
            low_threshold,
            medium_threshold,
            high_threshold,
            home_domain,
            signer,
        })
    }
}

impl SetOptionsOperationBuilder {
    pub fn new() -> SetOptionsOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> SetOptionsOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_inflation_destination(
        mut self,
        destination: Option<PublicKey>,
    ) -> SetOptionsOperationBuilder {
        self.inflation_destination = destination;
        self
    }

    pub fn with_clear_flags(mut self, flags: Option<AccountFlags>) -> SetOptionsOperationBuilder {
        self.clear_flags = flags;
        self
    }

    pub fn with_set_flags(mut self, flags: Option<AccountFlags>) -> SetOptionsOperationBuilder {
        self.set_flags = flags;
        self
    }

    pub fn with_master_weight(mut self, weight: Option<u32>) -> SetOptionsOperationBuilder {
        self.master_weight = weight;
        self
    }

    pub fn with_low_threshold(mut self, weight: Option<u32>) -> SetOptionsOperationBuilder {
        self.low_threshold = weight;
        self
    }
    pub fn with_medium_threshold(mut self, weight: Option<u32>) -> SetOptionsOperationBuilder {
        self.medium_threshold = weight;
        self
    }

    pub fn with_high_threshold(mut self, weight: Option<u32>) -> SetOptionsOperationBuilder {
        self.high_threshold = weight;
        self
    }

    pub fn with_signer(mut self, signer: Option<Signer>) -> SetOptionsOperationBuilder {
        self.signer = signer;
        self
    }

    pub fn build(self) -> Result<Operation> {
        Ok(Operation::SetOptions(SetOptionsOperation {
            source_account: self.source_account,
            inflation_destination: self.inflation_destination,
            clear_flags: self.clear_flags,
            set_flags: self.set_flags,
            master_weight: self.master_weight,
            low_threshold: self.low_threshold,
            medium_threshold: self.medium_threshold,
            high_threshold: self.high_threshold,
            home_domain: self.home_domain,
            signer: self.signer,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::account::AccountFlags;
    use crate::crypto::{Signer, SignerKey};
    use crate::network::Network;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_set_options() {
        let kp = keypair0();
        let kp1 = keypair1();

        let op = Operation::new_set_options()
            .with_inflation_destination(Some(kp1.public_key()))
            .with_set_flags(Some(
                AccountFlags::AUTH_REQUIRED | AccountFlags::AUTH_IMMUTABLE,
            ))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAFAAAAAQAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfgAAAAAAAAABAAAABQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB6i5yxQAAAEBtYhsjGguNMF06uqEn/cUIdy9eAp/X2jlhTRiVcIGUQJ2U/45eFGXZ8AjgE5P/fWoQYlsUihurccOMwu891EAD";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_set_options_with_source_account() {
        let kp = keypair0();
        let kp2 = keypair2();

        let op = Operation::new_set_options()
            .with_source_account(kp2.public_key())
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAfhHLNNY19eGrAtSgLD3VpaRm2AjNjxIBWQg9zS4VWZgAAAAFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHqLnLFAAAAQBf+wJNmYicge0JOI5iRVzprRG7AXpfQWCHRCIjqiXvJ0MRv71eSyPdJgUVlcStKM8prTF2TPuO8uWPk2kIRKAo=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_set_options_with_set_flags() {
        let kp = keypair0();

        let op = Operation::new_set_options()
            .with_set_flags(Some(
                AccountFlags::AUTH_REQUIRED | AccountFlags::AUTH_IMMUTABLE,
            ))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAFAAAAAAAAAAAAAAABAAAABQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB6i5yxQAAAECKOes75G40HX5JiVxydn+pu/DTZSGRf0A9eKdDXdS3Znog4kDjnw0vgZ7efMGl8NYW165N13sBub8Dnrc1E+MA";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_set_options_with_clear_flags() {
        let kp = keypair0();

        let op = Operation::new_set_options()
            .with_clear_flags(Some(
                AccountFlags::AUTH_REQUIRED | AccountFlags::AUTH_IMMUTABLE,
            ))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAFAAAAAAAAAAEAAAAFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB6i5yxQAAAEAXg+D6BNKJyRzCHu2np0PSWoAcFanuZa2gfS8a1iAB62buUwxezc/RULixb5W2rQwBxbSyaIrFA/3QJBf480UA";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_set_options_with_weights() {
        let kp = keypair0();

        let op = Operation::new_set_options()
            .with_master_weight(Some(1))
            .with_low_threshold(Some(2))
            .with_medium_threshold(Some(3))
            .with_high_threshold(Some(4))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAFAAAAAAAAAAAAAAAAAAAAAQAAAAEAAAABAAAAAgAAAAEAAAADAAAAAQAAAAQAAAAAAAAAAAAAAAAAAAAB6i5yxQAAAECfQ+WZfpgizILpZL84nvzoDM5+JMQOlA0+9FQZjj6Xr+njvLP/84HFz+lgK3/orX/1MdoBQb61sybrfC1kjdcA";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_set_options_with_signer() {
        let kp = keypair0();
        let kp2 = keypair2();

        let signer_key = SignerKey::Ed25519(kp2.public_key());
        let signer = Signer::new(signer_key, 8);

        let op = Operation::new_set_options()
            .with_signer(Some(signer))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAH4RyzTWNfXhqwLUoCw91aWkZtgIzY8SAVkIPc0uFVmYAAAACAAAAAAAAAAB6i5yxQAAAEDlGdxaTcfjFp4ukgepGrUe2ALXJZvDRBIGWw3ROBsQlxFV9kgx2YvszPy4DWtXQNvc3i0KxrUrR+r2liPGr/QJ";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
