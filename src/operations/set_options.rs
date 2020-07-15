use crate::account::AccountFlags;
use crate::amount::{Price, Stroops};
use crate::asset::Asset;
use crate::crypto::{MuxedAccount, PublicKey};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::signature::Signer;
use crate::xdr;
use std::convert::TryInto;

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

#[derive(Debug)]
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
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn inflation_destination(&self) -> &Option<PublicKey> {
        &self.inflation_destination
    }

    pub fn clear_flags(&self) -> &Option<AccountFlags> {
        &self.clear_flags
    }

    pub fn set_flags(&self) -> &Option<AccountFlags> {
        &self.set_flags
    }

    pub fn master_weight(&self) -> &Option<u32> {
        &self.master_weight
    }

    pub fn low_threshold(&self) -> &Option<u32> {
        &self.low_threshold
    }

    pub fn medium_threshold(&self) -> &Option<u32> {
        &self.medium_threshold
    }

    pub fn high_threshold(&self) -> &Option<u32> {
        &self.high_threshold
    }

    pub fn home_domain(&self) -> &Option<String> {
        &self.home_domain
    }

    pub fn signer(&self) -> &Option<Signer> {
        &self.signer
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inflation_dest = self
            .inflation_destination
            .as_ref()
            .map(|d| d.to_xdr_account_id())
            .transpose()?;
        let clear_flags = self.clear_flags.map(|f| xdr::Uint32::new(f.bits()));
        let set_flags = self.set_flags.map(|f| xdr::Uint32::new(f.bits()));
        let master_weight = self.master_weight.map(|w| xdr::Uint32::new(w));
        let low_threshold = self.low_threshold.map(|t| xdr::Uint32::new(t));
        let med_threshold = self.medium_threshold.map(|t| xdr::Uint32::new(t));
        let high_threshold = self.high_threshold.map(|t| xdr::Uint32::new(t));
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

    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::SetOptionsOp,
    ) -> Result<SetOptionsOperation> {
        let inflation_destination = x
            .inflation_dest
            .as_ref()
            .map(|d| PublicKey::from_xdr_account_id(d))
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
        let signer = x
            .signer
            .as_ref()
            .map(|s| Signer::from_xdr(&s))
            .transpose()?;
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
        SetOptionsOperationBuilder {
            source_account: None,
            inflation_destination: None,
            clear_flags: None,
            set_flags: None,
            master_weight: None,
            low_threshold: None,
            medium_threshold: None,
            high_threshold: None,
            home_domain: None,
            signer: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> SetOptionsOperationBuilder {
        self.source_account = Some(source);
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
