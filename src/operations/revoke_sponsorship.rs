use crate::asset::Asset;
use crate::claim::ClaimableBalanceId;
use crate::crypto::{MuxedAccount, PublicKey};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::signature::SignerKey;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevokeSponsorshipOperation {
    LedgerEntry(RevokeSponsorshipLedgerEntry),
    Signer(RevokeSponsorshipSigner),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeSponsorshipLedgerEntry {
    source_account: Option<MuxedAccount>,
    ledger_key: LedgerKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeSponsorshipSigner {
    source_account: Option<MuxedAccount>,
    account_id: PublicKey,
    signer_key: SignerKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerKey {
    Account(PublicKey),
    Trustline(PublicKey, Asset),
    Offer(PublicKey, i64),
    Data(PublicKey, String),
    ClaimableBalance(ClaimableBalanceId),
}

#[derive(Debug, Default)]
pub struct RevokeSponsorshipOperationBuilder {
    source_account: Option<MuxedAccount>,
    value: Option<RevokeSponsorshipValue>,
}

#[derive(Debug)]
pub enum RevokeSponsorshipValue {
    LedgerEntry(LedgerKey),
    Signer(PublicKey, SignerKey),
}

impl RevokeSponsorshipOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        match *self {
            RevokeSponsorshipOperation::LedgerEntry(ref le) => le.source_account(),
            RevokeSponsorshipOperation::Signer(ref s) => s.source_account(),
        }
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        match *self {
            RevokeSponsorshipOperation::LedgerEntry(ref mut le) => le.source_account_mut(),
            RevokeSponsorshipOperation::Signer(ref mut s) => s.source_account_mut(),
        }
    }

    /// If the operation is a LedgerEntry, returns its value. Returns None otherwise.
    pub fn as_ledger_entry(&self) -> Option<&RevokeSponsorshipLedgerEntry> {
        match *self {
            RevokeSponsorshipOperation::LedgerEntry(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// If the operation is a LedgerEntry, returns its value. Returns None otherwise.
    pub fn as_ledger_entry_mut(&mut self) -> Option<&mut RevokeSponsorshipLedgerEntry> {
        match *self {
            RevokeSponsorshipOperation::LedgerEntry(ref mut inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the operation is a LedgerEntry.
    pub fn is_ledger_entry(&self) -> bool {
        self.as_ledger_entry().is_some()
    }

    /// If the operation is a Signer, returns its value. Returns None otherwise.
    pub fn as_signer(&self) -> Option<&RevokeSponsorshipSigner> {
        match *self {
            RevokeSponsorshipOperation::Signer(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// If the operation is a Signer, returns its value. Returns None otherwise.
    pub fn as_signer_mut(&mut self) -> Option<&mut RevokeSponsorshipSigner> {
        match *self {
            RevokeSponsorshipOperation::Signer(ref mut inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns true if the operation is a Signer.
    pub fn is_signer(&self) -> bool {
        self.as_signer().is_some()
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inner = match *self {
            RevokeSponsorshipOperation::LedgerEntry(ref le) => {
                let ledger_key = le.ledger_key.to_xdr()?;
                xdr::RevokeSponsorshipOp::RevokeSponsorshipLedgerEntry(ledger_key)
            }
            RevokeSponsorshipOperation::Signer(ref s) => {
                let account_id = s.account_id.to_xdr_account_id()?;
                let signer_key = s.signer_key.to_xdr()?;
                let inner = xdr::RevokeSponsorshipOpSigner {
                    account_id,
                    signer_key,
                };
                xdr::RevokeSponsorshipOp::RevokeSponsorshipSigner(inner)
            }
        };
        Ok(xdr::OperationBody::RevokeSponsorship(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::RevokeSponsorshipOp,
    ) -> Result<RevokeSponsorshipOperation> {
        match x {
            xdr::RevokeSponsorshipOp::RevokeSponsorshipLedgerEntry(ref le) => {
                let ledger_key = LedgerKey::from_xdr(&le)?;
                let inner = RevokeSponsorshipLedgerEntry {
                    source_account,
                    ledger_key,
                };
                Ok(RevokeSponsorshipOperation::LedgerEntry(inner))
            }
            xdr::RevokeSponsorshipOp::RevokeSponsorshipSigner(ref s) => {
                let account_id = PublicKey::from_xdr_account_id(&s.account_id)?;
                let signer_key = SignerKey::from_xdr(&s.signer_key)?;
                let inner = RevokeSponsorshipSigner {
                    source_account,
                    account_id,
                    signer_key,
                };
                Ok(RevokeSponsorshipOperation::Signer(inner))
            }
        }
    }
}

impl RevokeSponsorshipLedgerEntry {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }
}

impl RevokeSponsorshipSigner {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }
}

impl LedgerKey {
    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::LedgerKey> {
        match *self {
            LedgerKey::Account(ref account_id) => {
                let account_id = account_id.to_xdr_account_id()?;
                let inner = xdr::LedgerKeyAccount { account_id };
                Ok(xdr::LedgerKey::Account(inner))
            }
            LedgerKey::Trustline(ref account_id, ref asset) => {
                let account_id = account_id.to_xdr_account_id()?;
                let asset = asset.to_xdr()?;
                let inner = xdr::LedgerKeyTrustLine { account_id, asset };
                Ok(xdr::LedgerKey::Trustline(inner))
            }
            LedgerKey::Offer(ref seller_id, ref offer_id) => {
                let seller_id = seller_id.to_xdr_account_id()?;
                let offer_id = xdr::Int64::new(*offer_id);
                let inner = xdr::LedgerKeyOffer {
                    seller_id,
                    offer_id,
                };
                Ok(xdr::LedgerKey::Offer(inner))
            }
            LedgerKey::Data(ref account_id, ref data_name) => {
                let account_id = account_id.to_xdr_account_id()?;
                let data_name = xdr::String64::new(data_name.to_string());
                let inner = xdr::LedgerKeyData {
                    account_id,
                    data_name,
                };
                Ok(xdr::LedgerKey::Data(inner))
            }
            LedgerKey::ClaimableBalance(ref balance_id) => {
                let balance_id = balance_id.to_xdr();
                let inner = xdr::LedgerKeyClaimableBalance { balance_id };
                Ok(xdr::LedgerKey::ClaimableBalance(inner))
            }
        }
    }

    /// Creates from the xdr object.
    pub fn from_xdr(x: &xdr::LedgerKey) -> Result<LedgerKey> {
        match x {
            xdr::LedgerKey::Account(ref account) => {
                let account_id = PublicKey::from_xdr_account_id(&account.account_id)?;
                Ok(LedgerKey::Account(account_id))
            }
            xdr::LedgerKey::Trustline(ref trustline) => {
                let account_id = PublicKey::from_xdr_account_id(&trustline.account_id)?;
                let asset = Asset::from_xdr(&trustline.asset)?;
                Ok(LedgerKey::Trustline(account_id, asset))
            }
            xdr::LedgerKey::Offer(ref offer) => {
                let seller_id = PublicKey::from_xdr_account_id(&offer.seller_id)?;
                let offer_id = offer.offer_id.value;
                Ok(LedgerKey::Offer(seller_id, offer_id))
            }
            xdr::LedgerKey::Data(ref data) => {
                let account_id = PublicKey::from_xdr_account_id(&data.account_id)?;
                let data_name = data.data_name.value.to_string();
                Ok(LedgerKey::Data(account_id, data_name))
            }
            xdr::LedgerKey::ClaimableBalance(ref claimable_balance) => {
                let balance_id = ClaimableBalanceId::from_xdr(&claimable_balance.balance_id)?;
                Ok(LedgerKey::ClaimableBalance(balance_id))
            }
        }
    }
}

impl RevokeSponsorshipOperationBuilder {
    pub fn new() -> RevokeSponsorshipOperationBuilder {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> RevokeSponsorshipOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_ledger_key(mut self, ledger_key: LedgerKey) -> RevokeSponsorshipOperationBuilder {
        self.value = Some(RevokeSponsorshipValue::LedgerEntry(ledger_key));
        self
    }

    pub fn with_signer(
        mut self,
        account_id: PublicKey,
        signer_key: SignerKey,
    ) -> RevokeSponsorshipOperationBuilder {
        self.value = Some(RevokeSponsorshipValue::Signer(account_id, signer_key));
        self
    }

    pub fn build(self) -> Result<Operation> {
        match self.value {
            None => Err(Error::InvalidOperation(
                "missing revoke sponsorship operation ledger key or signer".to_string(),
            )),
            Some(RevokeSponsorshipValue::LedgerEntry(ledger_key)) => {
                let ledger_entry = RevokeSponsorshipLedgerEntry {
                    source_account: self.source_account,
                    ledger_key,
                };
                let inner = RevokeSponsorshipOperation::LedgerEntry(ledger_entry);
                Ok(Operation::RevokeSponsorship(inner))
            }
            Some(RevokeSponsorshipValue::Signer(account_id, signer_key)) => {
                let signer = RevokeSponsorshipSigner {
                    source_account: self.source_account,
                    account_id,
                    signer_key,
                };
                let inner = RevokeSponsorshipOperation::Signer(signer);
                Ok(Operation::RevokeSponsorship(inner))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::asset::Asset;
    use crate::claim::ClaimableBalanceId;
    use crate::crypto::KeyPair;
    use crate::network::Network;
    use crate::operations::revoke_sponsorship::LedgerKey;
    use crate::operations::Operation;
    use crate::signature::SignerKey;
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
    fn test_revoke_sponsorship_ledger_key_account() {
        let kp = keypair0();
        let kp1 = keypair1();

        let op = Operation::new_revoke_sponsorship()
            .with_ledger_key(LedgerKey::Account(kp1.public_key().clone()))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAASAAAAAAAAAAAAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAAAAAAAAeoucsUAAABAlhwbGG2OC+ym0bD7G0GsGnbLInIVKzfLdhCl6AsyioseAydDXCVOB2A8Ywv4XfT0nC4BY26UdPBuLWG3cALmAg==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_revoke_sponsorship_ledger_key_trustline() {
        let kp = keypair0();
        let kp1 = keypair1();
        let kp2 = keypair2();

        let abcd = Asset::new_credit("ABCD", kp2.public_key().clone()).unwrap();

        let op = Operation::new_revoke_sponsorship()
            .with_ledger_key(LedgerKey::Trustline(kp1.public_key().clone(), abcd))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAASAAAAAAAAAAEAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAABQUJDRAAAAAB+Ecs01jX14asC1KAsPdWlpGbYCM2PEgFZCD3NLhVZmAAAAAAAAAAB6i5yxQAAAEA+1KnFKV7vhXjLxRJ+/aWfusVTrV3Az+Iscd13uKG0g6Pi41uTC5nsU07GeC2Os2bwz7r8XlNtxlkwHF89DCcG";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_revoke_sponsorship_ledger_key_offer() {
        let kp = keypair0();
        let kp1 = keypair1();

        let op = Operation::new_revoke_sponsorship()
            .with_ledger_key(LedgerKey::Offer(kp1.public_key().clone(), 123))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAASAAAAAAAAAAIAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAAAAAAAewAAAAAAAAAB6i5yxQAAAECBX+hYvz4LN3DoBmTTabB7aZGCjUqps1DZaMm9jLBsHgIUrfmoVNx2e0a6t1o0nvAKpatd3SCZFWIY0W6TnAYJ";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_revoke_sponsorship_ledger_key_data() {
        let kp = keypair0();
        let kp1 = keypair1();

        let op = Operation::new_revoke_sponsorship()
            .with_ledger_key(LedgerKey::Data(
                kp1.public_key().clone(),
                "Test_Data".to_string(),
            ))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAASAAAAAAAAAAMAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAAJVGVzdF9EYXRhAAAAAAAAAAAAAAHqLnLFAAAAQIEt621z4bNoQ9RXuT+bUktPySCRYocLfde5SKO2/94r4K8GBZhVzKBez80hNxfljncOuG4ZkzQ+mWaCGBjnpgE=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_revoke_sponsorship_ledger_key_balance_id() {
        let kp = keypair0();

        let balance_id = ClaimableBalanceId::new(vec![7; 32]).unwrap();

        let op = Operation::new_revoke_sponsorship()
            .with_ledger_key(LedgerKey::ClaimableBalance(balance_id))
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAASAAAAAAAAAAQAAAAABwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcAAAAAAAAAAeoucsUAAABAAofz60qpHGLrsNmcT9fgAnUOywCE5xDW8OMYpusgis1zODTg3fmsbFmUGB32DrGn+aeVtrLkVVjIY8vey3cVDQ==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_revoke_sponsorship_ledger_key_account_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();

        let op = Operation::new_revoke_sponsorship()
            .with_ledger_key(LedgerKey::Account(kp1.public_key().clone()))
            .with_source_account(kp1.public_key().clone())
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAASAAAAAAAAAAAAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAAAAAAAAeoucsUAAABA0Fd2Pp3NfMLTFGXbb6oks4IiwWQLqzQ71DFgVjv98Cle113hcH2toNlNEF7iT1D+262C4ajIhdAReZBubwTHCg==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_revoke_sponsorship_ledger_key_signer_key() {
        let kp = keypair0();
        let kp1 = keypair1();

        let signer_key = SignerKey::new_from_public_key(kp1.public_key().clone());

        let op = Operation::new_revoke_sponsorship()
            .with_signer(kp1.public_key().clone(), signer_key)
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAASAAAAAQAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfgAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfgAAAAAAAAAB6i5yxQAAAECkJuiFk6qWll/g4XtiknA2GktvVLIsQuxSs2SC2NvzrzxF0WSxZjOaKdDqeZ/AfkzgajS6mCo7s9e7sg9DcF8P";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_revoke_sponsorship_ledger_key_signer_key_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();

        let signer_key = SignerKey::new_from_public_key(kp1.public_key().clone());

        let op = Operation::new_revoke_sponsorship()
            .with_signer(kp1.public_key().clone(), signer_key)
            .with_source_account(kp1.public_key().clone())
            .build()
            .unwrap();
        let mut tx = Transaction::builder(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(&kp, &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAASAAAAAQAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfgAAAAAlyvHaD8duz+iEXkJUUbsHkklIlH46oMrMMYrt0odkfgAAAAAAAAAB6i5yxQAAAECe7rfndyOX8KE0jYOH5hH8oTYFF06UOEeQWvtLdxP9s0a/V8kTDclsyPpfCiC4dcNV5CPVifcolty05Qap2TUN";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
