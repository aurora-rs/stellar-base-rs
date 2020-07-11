use crate::amount::Stroops;
use crate::crypto::{hash, KeyPair, MuxedAccount, SecretKey};
use crate::error::{Error, Result};
use crate::memo::Memo;
use crate::network::Network;
use crate::operations::Operation;
use crate::signature::DecoratedSignature;
use crate::time_bounds::TimeBounds;
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

pub const MIN_BASE_FEE: Stroops = Stroops(100);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    source_account: MuxedAccount,
    fee: Stroops,
    sequence: i64,
    time_bounds: Option<TimeBounds>,
    memo: Memo,
    operations: Vec<Operation>,
    signatures: Vec<DecoratedSignature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeeBumpTransaction {
    fee_source: MuxedAccount,
    fee: Stroops,
    inner_tx: Transaction,
    signatures: Vec<DecoratedSignature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionEnvelope {
    Transaction(Transaction),
    FeeBumpTransaction(FeeBumpTransaction),
}

pub struct TransactionBuilder {
    source_account: MuxedAccount,
    sequence: i64,
    base_fee: Stroops,
    time_bounds: Option<TimeBounds>,
    memo: Memo,
    operations: Vec<Operation>,
}

pub fn transaction<S: Into<MuxedAccount>>(
    source_account: S,
    sequence: i64,
    fee: Stroops,
) -> TransactionBuilder {
    TransactionBuilder::new(source_account, sequence, fee)
}

pub fn fee_bump_transaction(
    fee_source: MuxedAccount,
    fee: Stroops,
    inner_tx: Transaction,
) -> FeeBumpTransaction {
    FeeBumpTransaction::new(fee_source, fee, inner_tx)
}

impl Transaction {
    pub fn source_account(&self) -> &MuxedAccount {
        &self.source_account
    }

    pub fn fee(&self) -> &Stroops {
        &self.fee
    }

    pub fn sequence(&self) -> &i64 {
        &self.sequence
    }

    pub fn time_bounds(&self) -> &Option<TimeBounds> {
        &self.time_bounds
    }

    pub fn memo(&self) -> &Memo {
        &self.memo
    }

    pub fn operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }

    pub fn to_envelope(self) -> TransactionEnvelope {
        TransactionEnvelope::Transaction(self)
    }

    pub fn sign(&mut self, key: &KeyPair, network: &Network) -> Result<()> {
        let signature = self.decorated_signature(&key, &network)?;
        self.signatures.push(signature);
        Ok(())
    }

    pub fn decorated_signature(
        &self,
        key: &KeyPair,
        network: &Network,
    ) -> Result<DecoratedSignature> {
        let tx_hash = self.hash(&network)?;
        Ok(key.sign_decorated(&tx_hash))
    }

    pub fn hash(&self, network: &Network) -> Result<Vec<u8>> {
        let signature_data = self.signature_data(network)?;
        Ok(hash(&signature_data))
    }

    pub fn signature_data(&self, network: &Network) -> Result<Vec<u8>> {
        let mut base = Vec::new();
        let tx_signature_payload = self.to_xdr_transaction_signature_payload(&network)?;
        tx_signature_payload
            .write_xdr(&mut base)
            .map_err(Error::XdrError)?;
        Ok(base)
    }

    pub fn to_xdr(&self) -> Result<xdr::Transaction> {
        let source_account = self.source_account.to_xdr()?;
        let fee = self.fee.to_xdr_uint32()?;
        let seq_num = xdr::SequenceNumber::new(xdr::Int64::new(self.sequence));
        let time_bounds = match &self.time_bounds {
            None => None,
            Some(tb) => Some(tb.to_xdr()?),
        };
        let memo = self.memo.to_xdr()?;
        let mut operations = Vec::new();
        for operation in self.operations() {
            let xdr_operation = operation.to_xdr()?;
            operations.push(xdr_operation);
        }
        let ext = xdr::TransactionExt::V0(());
        Ok(xdr::Transaction {
            source_account,
            fee,
            seq_num,
            time_bounds,
            memo,
            operations,
            ext,
        })
    }

    pub fn to_xdr_envelope(&self) -> Result<xdr::TransactionV1Envelope> {
        let tx = self.to_xdr()?;
        let signatures = signatures_to_xdr(self.signatures())?;
        Ok(xdr::TransactionV1Envelope { tx, signatures })
    }

    pub fn to_xdr_transaction_signature_payload(
        &self,
        network: &Network,
    ) -> Result<xdr::TransactionSignaturePayload> {
        let network_id = xdr::Hash::new(network.network_id());
        let inner = self.to_xdr()?;
        let tagged_transaction =
            xdr::TransactionSignaturePayloadTaggedTransaction::EnvelopeTypeTx(inner);
        Ok(xdr::TransactionSignaturePayload {
            network_id,
            tagged_transaction,
        })
    }

    pub fn from_xdr(x: &xdr::Transaction) -> Result<Transaction> {
        let source_account = MuxedAccount::from_xdr(&x.source_account)?;
        let fee = Stroops::from_xdr_uint32(&x.fee)?;
        let sequence = x.seq_num.value.value;
        let time_bounds = match &x.time_bounds {
            None => None,
            Some(tb) => Some(TimeBounds::from_xdr(tb)?),
        };
        let memo = Memo::from_xdr(&x.memo)?;
        let mut operations = Vec::new();
        for operation in &x.operations {
            let xdr_operation = Operation::from_xdr(&operation)?;
            operations.push(xdr_operation);
        }
        Ok(Transaction {
            source_account,
            fee,
            sequence,
            time_bounds,
            memo,
            operations,
            signatures: Vec::new(),
        })
    }

    pub fn from_xdr_envelope(x: &xdr::TransactionV1Envelope) -> Result<Transaction> {
        let mut tx = Self::from_xdr(&x.tx)?;
        let signatures = signatures_from_xdr(&x.signatures)?;
        tx.signatures = signatures;
        Ok(tx)
    }
}

impl FeeBumpTransaction {
    pub fn new(
        fee_source: MuxedAccount,
        fee: Stroops,
        inner_tx: Transaction,
    ) -> FeeBumpTransaction {
        FeeBumpTransaction {
            fee_source,
            fee,
            inner_tx,
            signatures: Vec::new(),
        }
    }

    pub fn fee_source(&self) -> &MuxedAccount {
        &self.fee_source
    }

    pub fn fee(&self) -> &Stroops {
        &self.fee
    }

    pub fn inner_transaction(&self) -> &Transaction {
        &self.inner_tx
    }

    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }

    pub fn to_envelope(self) -> TransactionEnvelope {
        TransactionEnvelope::FeeBumpTransaction(self)
    }

    pub fn sign(&mut self, key: &KeyPair, network: &Network) -> Result<()> {
        let signature = self.decorated_signature(&key, &network)?;
        self.signatures.push(signature);
        Ok(())
    }

    pub fn decorated_signature(
        &self,
        key: &KeyPair,
        network: &Network,
    ) -> Result<DecoratedSignature> {
        let tx_hash = self.hash(&network)?;
        Ok(key.sign_decorated(&tx_hash))
    }

    pub fn hash(&self, network: &Network) -> Result<Vec<u8>> {
        let signature_data = self.signature_data(network)?;
        Ok(hash(&signature_data))
    }

    pub fn signature_data(&self, network: &Network) -> Result<Vec<u8>> {
        let mut base = Vec::new();
        let tx_signature_payload = self.to_xdr_transaction_signature_payload(&network)?;
        tx_signature_payload
            .write_xdr(&mut base)
            .map_err(Error::XdrError)?;
        Ok(base)
    }

    pub fn to_xdr(&self) -> Result<xdr::FeeBumpTransaction> {
        let fee_source = self.fee_source.to_xdr()?;
        let fee = self.fee.to_xdr_int64()?;
        let tx_envelope = self.inner_tx.to_xdr_envelope()?;
        let inner_tx = xdr::FeeBumpTransactionInnerTx::EnvelopeTypeTx(tx_envelope);
        let ext = xdr::FeeBumpTransactionExt::V0(());
        Ok(xdr::FeeBumpTransaction {
            fee_source,
            fee,
            inner_tx,
            ext,
        })
    }

    pub fn to_xdr_envelope(&self) -> Result<xdr::FeeBumpTransactionEnvelope> {
        let tx = self.to_xdr()?;
        let signatures = signatures_to_xdr(self.signatures())?;
        Ok(xdr::FeeBumpTransactionEnvelope { tx, signatures })
    }

    pub fn from_xdr(x: &xdr::FeeBumpTransaction) -> Result<FeeBumpTransaction> {
        let fee_source = MuxedAccount::from_xdr(&x.fee_source)?;
        let fee = Stroops::new(x.fee.value);
        let inner_tx = match &x.inner_tx {
            xdr::FeeBumpTransactionInnerTx::EnvelopeTypeTx(inner_tx) => {
                Transaction::from_xdr_envelope(&inner_tx)?
            }
        };
        Ok(FeeBumpTransaction {
            fee_source,
            fee,
            inner_tx,
            signatures: Vec::new(),
        })
    }

    pub fn from_xdr_envelope(x: &xdr::FeeBumpTransactionEnvelope) -> Result<FeeBumpTransaction> {
        let mut tx = FeeBumpTransaction::from_xdr(&x.tx)?;
        let signatures = signatures_from_xdr(&x.signatures)?;
        tx.signatures = signatures;
        Ok(tx)
    }

    pub fn to_xdr_transaction_signature_payload(
        &self,
        network: &Network,
    ) -> Result<xdr::TransactionSignaturePayload> {
        let network_id = xdr::Hash::new(network.network_id());
        let inner = self.to_xdr()?;
        let tagged_transaction =
            xdr::TransactionSignaturePayloadTaggedTransaction::EnvelopeTypeTxFeeBump(inner);
        Ok(xdr::TransactionSignaturePayload {
            network_id,
            tagged_transaction,
        })
    }
}

impl TransactionEnvelope {
    pub fn sign(&mut self, key: &KeyPair, network: &Network) -> Result<()> {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.sign(&key, &network),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.sign(&key, &network),
        }
    }

    pub fn decorated_signature(
        &self,
        key: &KeyPair,
        network: &Network,
    ) -> Result<DecoratedSignature> {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.decorated_signature(&key, &network),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.decorated_signature(&key, &network),
        }
    }

    pub fn hash(&self, network: &Network) -> Result<Vec<u8>> {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.hash(&network),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.hash(&network),
        }
    }

    pub fn signature_data(&self, network: &Network) -> Result<Vec<u8>> {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.signature_data(&network),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.signature_data(&network),
        }
    }

    pub fn to_xdr(&self) -> Result<xdr::TransactionEnvelope> {
        match self {
            TransactionEnvelope::Transaction(tx) => {
                let inner = tx.to_xdr_envelope()?;
                Ok(xdr::TransactionEnvelope::EnvelopeTypeTx(inner))
            }
            TransactionEnvelope::FeeBumpTransaction(tx) => {
                let inner = tx.to_xdr_envelope()?;
                Ok(xdr::TransactionEnvelope::EnvelopeTypeTxFeeBump(inner))
            }
        }
    }

    pub fn from_xdr(x: &xdr::TransactionEnvelope) -> Result<TransactionEnvelope> {
        match x {
            xdr::TransactionEnvelope::EnvelopeTypeTx(inner) => {
                let tx = Transaction::from_xdr_envelope(inner)?;
                Ok(TransactionEnvelope::Transaction(tx))
            }
            xdr::TransactionEnvelope::EnvelopeTypeTxV0(_) => todo!(),
            xdr::TransactionEnvelope::EnvelopeTypeTxFeeBump(inner) => {
                let tx = FeeBumpTransaction::from_xdr_envelope(inner)?;
                Ok(TransactionEnvelope::FeeBumpTransaction(tx))
            }
        }
    }

    pub fn to_xdr_transaction_signature_payload(
        &self,
        network: &Network,
    ) -> Result<xdr::TransactionSignaturePayload> {
        match self {
            TransactionEnvelope::Transaction(tx) => {
                tx.to_xdr_transaction_signature_payload(&network)
            }
            TransactionEnvelope::FeeBumpTransaction(tx) => {
                tx.to_xdr_transaction_signature_payload(&network)
            }
        }
    }
}

impl TransactionBuilder {
    pub fn new<S: Into<MuxedAccount>>(
        source_account: S,
        sequence: i64,
        base_fee: Stroops,
    ) -> TransactionBuilder {
        TransactionBuilder {
            source_account: source_account.into(),
            sequence,
            base_fee,
            time_bounds: None,
            memo: Memo::none(),
            operations: Vec::new(),
        }
    }

    pub fn with_time_bounds(mut self, time_bounds: TimeBounds) -> TransactionBuilder {
        self.time_bounds = Some(time_bounds);
        self
    }

    pub fn with_memo(mut self, memo: Memo) -> TransactionBuilder {
        self.memo = memo;
        self
    }

    pub fn add_operation(mut self, operation: Operation) -> TransactionBuilder {
        self.operations.push(operation);
        self
    }

    pub fn to_transaction(self) -> Result<Transaction> {
        if self.operations.len() > xdr::MAX_OPS_PER_TX as usize {
            return Err(Error::TooManyOperations);
        }
        if self.operations.len() == 0 {
            return Err(Error::MissingOperations);
        }

        if self.base_fee < MIN_BASE_FEE {
            return Err(Error::TransactionFeeTooLow);
        }

        self.to_transaction_unchecked()
    }

    pub fn to_transaction_unchecked(self) -> Result<Transaction> {
        let fee = self
            .base_fee
            .0
            .checked_mul(self.operations.len() as i64)
            .ok_or_else(|| Error::TransactionFeeOverflow)?;

        Ok(Transaction {
            source_account: self.source_account,
            sequence: self.sequence,
            fee: Stroops::new(fee),
            time_bounds: self.time_bounds,
            memo: self.memo,
            operations: self.operations,
            signatures: Vec::new(),
        })
    }
}

impl XDRSerialize for TransactionEnvelope {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr_tx = self.to_xdr()?;
        xdr_tx.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for TransactionEnvelope {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_tx, bytes_read) =
            xdr::TransactionEnvelope::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = TransactionEnvelope::from_xdr(&xdr_tx)?;
        Ok((res, bytes_read))
    }
}

fn signatures_to_xdr(signatures: &Vec<DecoratedSignature>) -> Result<Vec<xdr::DecoratedSignature>> {
    let mut xdr_signatures = Vec::new();
    for signature in signatures {
        let xdr_signature = signature.to_xdr()?;
        xdr_signatures.push(xdr_signature);
    }
    Ok(xdr_signatures)
}

fn signatures_from_xdr(
    xdr_signatures: &Vec<xdr::DecoratedSignature>,
) -> Result<Vec<DecoratedSignature>> {
    let mut signatures = Vec::new();
    for xdr_signature in xdr_signatures {
        let signature = DecoratedSignature::from_xdr(&xdr_signature)?;
        signatures.push(signature);
    }
    Ok(signatures)
}

#[cfg(test)]
mod tests {
    use super::{transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::amount::{Amount, Stroops};
    use crate::crypto::KeyPair;
    use crate::memo::Memo;
    use crate::network::Network;
    use crate::operations;
    use crate::time_bounds::TimeBounds;
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use std::str::FromStr;

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
    fn test_transaction_builder() {
        let kp = KeyPair::random().unwrap();
        let tx = transaction(kp.public_key().clone(), 123, Stroops::new(100))
            .with_memo(Memo::id(987))
            .with_time_bounds(TimeBounds::always_valid())
            .add_operation(operations::inflation().build())
            .to_transaction()
            .unwrap();
        assert_eq!(123, *tx.sequence());
        assert_eq!(&Stroops::new(100), tx.fee());
        assert!(tx.memo().is_id());
        assert!(tx.time_bounds().is_some());
        assert_eq!(1, tx.operations().len());
    }

    #[test]
    fn test_inflation() {
        let kp = keypair0();
        let mut tx = transaction(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(operations::inflation().build())
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::test());
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAJAAAAAAAAAAHqLnLFAAAAQCvHHPKuTRaRXk9BH05oWii0PJRmVOoqMxxg+79MLO90n1ljVNoaQ1Fliy8Xe34yfUzjhMB/TCXH29T8dTYtBg4=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_create_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let dest = kp1.public_key();
        let starting_balance = Amount::from_str("12.30").unwrap();

        let op = operations::create_account()
            .with_destination(dest.clone())
            .with_starting_balance(starting_balance)
            .unwrap()
            .build()
            .unwrap();
        let mut tx = transaction(kp.public_key().clone(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .to_transaction()
            .unwrap();
        tx.sign(&kp, &Network::test());
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAACXK8doPx27P6IReQlRRuweSSUiUfjqgyswxiu3Sh2R+AAAAAAdU1MAAAAAAAAAAAeoucsUAAABA0LiVS5BXQiPx/ZkMiJ55RngpeurtEgOrqbzAy99ZGnLUh68uiBejtKJdJPlw4XmVP/kojrA6nLI00zXhUiI7AQ==";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
