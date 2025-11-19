//! Transaction that changes the ledger state.
use std::io::{Read, Write};

use crate::amount::Stroops;
use crate::crypto::{
    hash, DecoratedSignature, Ed25519Signer, Ed25519Verifier, KeyPair, MuxedAccount,
};
use crate::error::{Error, Result};
use crate::memo::Memo;
use crate::network::Network;
use crate::operations::Operation;
use crate::time_bounds::TimeBounds;
use crate::{xdr, PublicKey};
use ed25519::Signature;

/// Minimum base fee.
pub const MIN_BASE_FEE: Stroops = Stroops(100);

/// Stellar transaction.
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

/// Fee bump transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeeBumpTransaction {
    fee_source: MuxedAccount,
    fee: Stroops,
    inner_tx: Transaction,
    signatures: Vec<DecoratedSignature>,
}

/// Transaction envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionEnvelope {
    /// Transaction
    Transaction(Transaction),
    /// Fee bump transaction.
    FeeBumpTransaction(FeeBumpTransaction),
}

/// A builder to construct the properties of a `Transaction`.
pub struct TransactionBuilder {
    base_fee: Stroops,
    tx: Result<Transaction>,
}

impl Transaction {
    /// Creates a `TransactionBuilder` to configure a `Transaction`.
    ///
    /// This is the same as `TransactionBuilder::new`.
    pub fn builder<S: Into<MuxedAccount>>(
        source_account: S,
        sequence: i64,
        fee: Stroops,
    ) -> TransactionBuilder {
        TransactionBuilder::new(source_account, sequence, fee)
    }

    /// Retrieves the transaction source account.
    pub fn source_account(&self) -> &MuxedAccount {
        &self.source_account
    }

    /// Retrieves a mutable reference to the transaction source account.
    pub fn source_account_mut(&mut self) -> &mut MuxedAccount {
        &mut self.source_account
    }

    /// Retrieves the transaction fee.
    pub fn fee(&self) -> &Stroops {
        &self.fee
    }

    /// Retrieves a mutable reference to the transaction fee.
    pub fn fee_mut(&mut self) -> &mut Stroops {
        &mut self.fee
    }

    /// Retrieves the transaction sequence number.
    pub fn sequence(&self) -> &i64 {
        &self.sequence
    }

    /// Retrieves a mutable reference to the transaction sequence number.
    pub fn sequence_mut(&mut self) -> &mut i64 {
        &mut self.sequence
    }

    /// Retrieves the transaction time bounds.
    pub fn time_bounds(&self) -> &Option<TimeBounds> {
        &self.time_bounds
    }

    /// Retrieves a mutable reference to the transaction time bounds.
    pub fn time_bounds_mut(&mut self) -> &mut Option<TimeBounds> {
        &mut self.time_bounds
    }

    /// Retrieves the transaction memo.
    pub fn memo(&self) -> &Memo {
        &self.memo
    }

    /// Retrieves a mutable reference to the transaction memo.
    pub fn memo_mut(&mut self) -> &mut Memo {
        &mut self.memo
    }

    /// Retrieves the transaction operations.
    pub fn operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    /// Retrieves a mutable reference to the transaction operations.
    pub fn operations_mut(&mut self) -> &mut Vec<Operation> {
        &mut self.operations
    }

    /// Retrieves the transaction signatures.
    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }

    /// Retrieves a mutable reference to the transaction signatures.
    pub fn signatures_mut(&mut self) -> &mut Vec<DecoratedSignature> {
        &mut self.signatures
    }

    /// Creates a `TransactionEnvelope` from the transaction.
    pub fn to_envelope(&self) -> TransactionEnvelope {
        self.clone().into_envelope()
    }

    /// Creates a `TransactionEnvelope` from the transaction.
    ///
    /// This consumes the transaction and takes ownership of it.
    pub fn into_envelope(self) -> TransactionEnvelope {
        TransactionEnvelope::Transaction(self)
    }

    /// Sign transaction with `preimage`, and add signature.
    ///
    /// This signs the transaction with the preimage `x` of `hash(x)`.
    pub fn sign_hashx(&mut self, preimage: &[u8]) -> Result<()> {
        let signature = self.decorated_signature_from_preimage(preimage)?;
        self.signatures.push(signature);
        Ok(())
    }

    /// Sign transaction with `key` for `network`, and add signature.
    pub fn sign<S, V>(&mut self, key: &KeyPair<S, V>, network: &Network) -> Result<()>
    where
        S: Ed25519Signer<Signature>,
        V: Ed25519Verifier<Signature> + AsRef<[u8]>,
    {
        let signature = self.decorated_signature(key, network)?;
        self.signatures.push(signature);
        Ok(())
    }

    /// Returns the decorated signature of the transaction create with `image`.
    pub fn decorated_signature_from_preimage(&self, preimage: &[u8]) -> Result<DecoratedSignature> {
        DecoratedSignature::new_from_preimage(preimage)
    }

    /// Returns the decorated signature of the transaction create with `key` for `network`.
    pub fn decorated_signature<S, V>(
        &self,
        key: &KeyPair<S, V>,
        network: &Network,
    ) -> Result<DecoratedSignature>
    where
        S: Ed25519Signer<Signature>,
        V: Ed25519Verifier<Signature> + AsRef<[u8]>,
    {
        let tx_hash = self.hash(network)?;
        Ok(key.sign_decorated(&tx_hash))
    }

    /// Returns the transaction hash for the transaction on `network`.
    pub fn hash(&self, network: &Network) -> Result<Vec<u8>> {
        let signature_data = self.signature_data(network)?;
        Ok(hash(&signature_data))
    }

    /// Returns the transaction signature data as bytes.
    pub fn signature_data(&self, network: &Network) -> Result<Vec<u8>> {
        let mut base = Vec::new();
        let tx_signature_payload = self.to_xdr_transaction_signature_payload(network)?;
        xdr::XDRSerialize::write_xdr(&tx_signature_payload, &mut base)?;
        Ok(base)
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::Transaction> {
        let source_account = self.source_account.to_xdr()?;
        let fee = self.fee.to_xdr_uint32()?;
        let seq_num = xdr::SequenceNumber(self.sequence);
        let cond = match &self.time_bounds {
            None => xdr::Preconditions::None,
            Some(tb) => xdr::Preconditions::Time(tb.to_xdr()?),
        };
        let memo = self.memo.to_xdr()?;
        let mut operations = Vec::new();
        for operation in self.operations() {
            let xdr_operation = operation.to_xdr()?;
            operations.push(xdr_operation);
        }
        let ext = xdr::TransactionExt::V0;
        Ok(xdr::Transaction {
            source_account,
            fee,
            seq_num,
            cond,
            memo,
            operations: operations.try_into().map_err(|_| Error::XdrError)?,
            ext,
        })
    }

    /// Returns the transaction envelope v1 xdr object.
    pub fn to_xdr_envelope(&self) -> Result<xdr::TransactionV1Envelope> {
        let tx = self.to_xdr()?;
        let signatures = signatures_to_xdr(self.signatures())?;
        Ok(xdr::TransactionV1Envelope { tx, signatures })
    }

    /// Returns the xdr transaction signature payload object.
    pub fn to_xdr_transaction_signature_payload(
        &self,
        network: &Network,
    ) -> Result<xdr::TransactionSignaturePayload> {
        let network_id = network
            .network_id()
            .try_into()
            .map_err(|_| Error::XdrError)?;
        let inner = self.to_xdr()?;
        let tagged_transaction = xdr::TransactionSignaturePayloadTaggedTransaction::Tx(inner);
        Ok(xdr::TransactionSignaturePayload {
            network_id,
            tagged_transaction,
        })
    }

    /// Creates from v0 xdr object.
    pub fn from_xdr_v0(x: &xdr::TransactionV0) -> Result<Transaction> {
        let source_account =
            MuxedAccount::Ed25519(PublicKey::from_slice(&x.source_account_ed25519.0)?);
        let fee = Stroops::from_xdr_uint32(x.fee)?;
        let sequence = x.seq_num.0;
        let time_bounds = match &x.time_bounds {
            None => None,
            Some(tb) => Some(TimeBounds::from_xdr(tb)?),
        };
        let memo = Memo::from_xdr(&x.memo)?;
        let mut operations = Vec::new();
        for operation in x.operations.as_slice() {
            let xdr_operation = Operation::from_xdr(operation)?;
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

    /// Creates from xdr object.
    pub fn from_xdr(x: &xdr::Transaction) -> Result<Transaction> {
        let source_account = MuxedAccount::from_xdr(&x.source_account)?;
        let fee = Stroops::from_xdr_uint32(x.fee)?;
        let sequence = x.seq_num.0;
        let time_bounds = match &x.cond {
            xdr::Preconditions::None => None,
            xdr::Preconditions::Time(tb) => Some(TimeBounds::from_xdr(tb)?),
            xdr::Preconditions::V2(_) => return Err(Error::UnsupportedFeature),
        };
        let memo = Memo::from_xdr(&x.memo)?;
        let mut operations = Vec::new();
        for operation in x.operations.as_slice() {
            let xdr_operation = Operation::from_xdr(operation)?;
            operations.push(xdr_operation);
        }
        match &x.ext {
            xdr::TransactionExt::V0 => {}
            xdr::TransactionExt::V1(_) => return Err(Error::UnsupportedFeature),
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

    /// Creates from xdr v0 envelope object.
    pub fn from_xdr_v0_envelope(x: &xdr::TransactionV0Envelope) -> Result<Transaction> {
        let mut tx = Self::from_xdr_v0(&x.tx)?;
        let signatures = signatures_from_xdr(&x.signatures)?;
        tx.signatures = signatures;
        Ok(tx)
    }

    /// Creates from xdr envelope object.
    pub fn from_xdr_envelope(x: &xdr::TransactionV1Envelope) -> Result<Transaction> {
        let mut tx = Self::from_xdr(&x.tx)?;
        let signatures = signatures_from_xdr(&x.signatures)?;
        tx.signatures = signatures;
        Ok(tx)
    }
}

impl FeeBumpTransaction {
    /// Creates a new fee bump transaction.
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

    /// Retrieves the transaction fee source.
    pub fn fee_source(&self) -> &MuxedAccount {
        &self.fee_source
    }

    /// Retrieves a mutable reference to the transaction fee source.
    pub fn fee_source_mut(&mut self) -> &mut MuxedAccount {
        &mut self.fee_source
    }

    /// Retrievies the transaction fee.
    pub fn fee(&self) -> &Stroops {
        &self.fee
    }

    /// Retrievies a mutable reference to the transaction fee.
    pub fn fee_mut(&mut self) -> &mut Stroops {
        &mut self.fee
    }

    /// Retrieves the transaction inner transaction.
    pub fn inner_transaction(&self) -> &Transaction {
        &self.inner_tx
    }

    /// Retrieves a mutable reference to the transaction inner transaction.
    pub fn inner_transaction_mut(&mut self) -> &mut Transaction {
        &mut self.inner_tx
    }

    /// Retrieves the transaction signatures.
    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }

    /// Retrieves a mutable reference the transaction signatures.
    pub fn signatures_mut(&mut self) -> &mut Vec<DecoratedSignature> {
        &mut self.signatures
    }

    /// Creates a `TransactionEnvelope` from the transaction.
    ///
    /// This consumes the transaction and takes ownership of it.
    pub fn into_envelope(self) -> TransactionEnvelope {
        TransactionEnvelope::FeeBumpTransaction(self)
    }

    /// Creates a `TransactionEnvelope` from the transaction.
    pub fn to_envelope(&self) -> TransactionEnvelope {
        self.clone().into_envelope()
    }

    /// Sign transaction with `preimage`, and add signature.
    ///
    /// This signs the transaction with the preimage `x` of `hash(x)`.
    pub fn sign_hashx(&mut self, preimage: &[u8]) -> Result<()> {
        let signature = self.decorated_signature_from_preimage(preimage)?;
        self.signatures.push(signature);
        Ok(())
    }

    /// Sign transaction with `key` for `network`, and add signature.
    pub fn sign<S, V>(&mut self, key: &KeyPair<S, V>, network: &Network) -> Result<()>
    where
        S: Ed25519Signer<Signature>,
        V: Ed25519Verifier<Signature> + AsRef<[u8]>,
    {
        let signature = self.decorated_signature(key, network)?;
        self.signatures.push(signature);
        Ok(())
    }

    /// Returns the decorated signature of the transaction create with `image`.
    pub fn decorated_signature_from_preimage(&self, preimage: &[u8]) -> Result<DecoratedSignature> {
        DecoratedSignature::new_from_preimage(preimage)
    }

    /// Returns the decorated signature of the transaction create with `key` for `network`.
    pub fn decorated_signature<S, V>(
        &self,
        key: &KeyPair<S, V>,
        network: &Network,
    ) -> Result<DecoratedSignature>
    where
        S: Ed25519Signer<Signature>,
        V: Ed25519Verifier<Signature> + AsRef<[u8]>,
    {
        let tx_hash = self.hash(network)?;
        Ok(key.sign_decorated(&tx_hash))
    }

    /// Returns the transaction hash for the transaction on `network`.
    pub fn hash(&self, network: &Network) -> Result<Vec<u8>> {
        let signature_data = self.signature_data(network)?;
        Ok(hash(&signature_data))
    }

    /// Returns the transaction signature data as bytes.
    pub fn signature_data(&self, network: &Network) -> Result<Vec<u8>> {
        let mut base = Vec::new();
        let tx_signature_payload = self.to_xdr_transaction_signature_payload(network)?;
        xdr::XDRSerialize::write_xdr(&tx_signature_payload, &mut base)?;
        Ok(base)
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::FeeBumpTransaction> {
        let fee_source = self.fee_source.to_xdr()?;
        let fee = self.fee.to_xdr_int64()?;
        let tx_envelope = self.inner_tx.to_xdr_envelope()?;
        let inner_tx = xdr::FeeBumpTransactionInnerTx::Tx(tx_envelope);
        let ext = xdr::FeeBumpTransactionExt::V0;
        Ok(xdr::FeeBumpTransaction {
            fee_source,
            fee,
            inner_tx,
            ext,
        })
    }

    /// Returns the fee bump transaction envelope xdr object.
    pub fn to_xdr_envelope(&self) -> Result<xdr::FeeBumpTransactionEnvelope> {
        let tx = self.to_xdr()?;
        let signatures = signatures_to_xdr(self.signatures())?;
        Ok(xdr::FeeBumpTransactionEnvelope { tx, signatures })
    }

    /// Creates from xdr object.
    pub fn from_xdr(x: &xdr::FeeBumpTransaction) -> Result<FeeBumpTransaction> {
        let fee_source = MuxedAccount::from_xdr(&x.fee_source)?;
        let fee = Stroops::new(x.fee);
        let inner_tx = match &x.inner_tx {
            xdr::FeeBumpTransactionInnerTx::Tx(inner_tx) => {
                Transaction::from_xdr_envelope(inner_tx)?
            }
        };
        Ok(FeeBumpTransaction {
            fee_source,
            fee,
            inner_tx,
            signatures: Vec::new(),
        })
    }

    /// Creates from xdr envelope object.
    pub fn from_xdr_envelope(x: &xdr::FeeBumpTransactionEnvelope) -> Result<FeeBumpTransaction> {
        let mut tx = FeeBumpTransaction::from_xdr(&x.tx)?;
        let signatures = signatures_from_xdr(&x.signatures)?;
        tx.signatures = signatures;
        Ok(tx)
    }

    /// Returns the xdr transaction signature payload object.
    pub fn to_xdr_transaction_signature_payload(
        &self,
        network: &Network,
    ) -> Result<xdr::TransactionSignaturePayload> {
        let network_id = network
            .network_id()
            .try_into()
            .map_err(|_| Error::XdrError)?;
        let inner = self.to_xdr()?;
        let tagged_transaction =
            xdr::TransactionSignaturePayloadTaggedTransaction::TxFeeBump(inner);
        Ok(xdr::TransactionSignaturePayload {
            network_id,
            tagged_transaction,
        })
    }
}

impl TransactionEnvelope {
    /// If the transaction is a Transaction, returns its value. Returns None otherwise.
    pub fn as_transaction(&self) -> Option<&Transaction> {
        match *self {
            TransactionEnvelope::Transaction(ref tx) => Some(tx),
            _ => None,
        }
    }

    /// If the transaction is a Transaction, returns its mutable value. Returns None otherwise.
    pub fn as_transaction_mut(&mut self) -> Option<&mut Transaction> {
        match *self {
            TransactionEnvelope::Transaction(ref mut tx) => Some(tx),
            _ => None,
        }
    }

    /// Returns true if the transaction is a Transaction.
    pub fn is_transaction(&self) -> bool {
        self.as_transaction().is_some()
    }

    /// If the transaction is a FeeBumpTransaction, returns its value. Returns None otherwise.
    pub fn as_fee_bump_transaction(&self) -> Option<&FeeBumpTransaction> {
        match *self {
            TransactionEnvelope::FeeBumpTransaction(ref tx) => Some(tx),
            _ => None,
        }
    }

    /// If the transaction is a FeeBumpTransaction, returns its mutable value. Returns None otherwise.
    pub fn as_fee_bump_transaction_mut(&mut self) -> Option<&mut FeeBumpTransaction> {
        match *self {
            TransactionEnvelope::FeeBumpTransaction(ref mut tx) => Some(tx),
            _ => None,
        }
    }

    /// Returns true if the transaction is a FeeBumpTransaction.
    pub fn is_fee_bump_transaction(&self) -> bool {
        self.as_fee_bump_transaction().is_some()
    }

    /// Sign transaction with `preimage`, and add signature.
    ///
    /// This signs the transaction with the preimage `x` of `hash(x)`.
    pub fn sign_hashx(&mut self, preimage: &[u8]) -> Result<()> {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.sign_hashx(preimage),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.sign_hashx(preimage),
        }
    }

    /// Sign transaction with `key` for `network`, and add signature.
    pub fn sign<S, V>(&mut self, key: &KeyPair<S, V>, network: &Network) -> Result<()>
    where
        S: Ed25519Signer<Signature>,
        V: Ed25519Verifier<Signature> + AsRef<[u8]>,
    {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.sign(key, network),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.sign(key, network),
        }
    }

    /// Returns the decorated signature of the transaction create with `image`.
    pub fn decorated_signature_from_preimage(&self, preimage: &[u8]) -> Result<DecoratedSignature> {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.decorated_signature_from_preimage(preimage),
            TransactionEnvelope::FeeBumpTransaction(tx) => {
                tx.decorated_signature_from_preimage(preimage)
            }
        }
    }

    /// Returns the decorated signature of the transaction create with `key` for `network`.
    pub fn decorated_signature<S, V>(
        &self,
        key: &KeyPair<S, V>,
        network: &Network,
    ) -> Result<DecoratedSignature>
    where
        S: Ed25519Signer<Signature>,
        V: Ed25519Verifier<Signature> + AsRef<[u8]>,
    {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.decorated_signature(key, network),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.decorated_signature(key, network),
        }
    }

    /// Returns the transaction hash for the transaction on `network`.
    pub fn hash(&self, network: &Network) -> Result<Vec<u8>> {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.hash(network),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.hash(network),
        }
    }

    /// Returns the transaction signature data as bytes.
    pub fn signature_data(&self, network: &Network) -> Result<Vec<u8>> {
        match self {
            TransactionEnvelope::Transaction(tx) => tx.signature_data(network),
            TransactionEnvelope::FeeBumpTransaction(tx) => tx.signature_data(network),
        }
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::TransactionEnvelope> {
        match self {
            TransactionEnvelope::Transaction(tx) => {
                let inner = tx.to_xdr_envelope()?;
                Ok(xdr::TransactionEnvelope::Tx(inner))
            }
            TransactionEnvelope::FeeBumpTransaction(tx) => {
                let inner = tx.to_xdr_envelope()?;
                Ok(xdr::TransactionEnvelope::TxFeeBump(inner))
            }
        }
    }

    /// Creates from xdr object.
    pub fn from_xdr(x: &xdr::TransactionEnvelope) -> Result<TransactionEnvelope> {
        match x {
            xdr::TransactionEnvelope::Tx(inner) => {
                let tx = Transaction::from_xdr_envelope(inner)?;
                Ok(TransactionEnvelope::Transaction(tx))
            }
            xdr::TransactionEnvelope::TxV0(inner) => {
                let tx = Transaction::from_xdr_v0_envelope(inner)?;
                Ok(TransactionEnvelope::Transaction(tx))
            }
            xdr::TransactionEnvelope::TxFeeBump(inner) => {
                let tx = FeeBumpTransaction::from_xdr_envelope(inner)?;
                Ok(TransactionEnvelope::FeeBumpTransaction(tx))
            }
        }
    }

    /// Returns the xdr transaction signature payload object.
    pub fn to_xdr_transaction_signature_payload(
        &self,
        network: &Network,
    ) -> Result<xdr::TransactionSignaturePayload> {
        match self {
            TransactionEnvelope::Transaction(tx) => {
                tx.to_xdr_transaction_signature_payload(network)
            }
            TransactionEnvelope::FeeBumpTransaction(tx) => {
                tx.to_xdr_transaction_signature_payload(network)
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
        let tx = Transaction {
            source_account: source_account.into(),
            sequence,
            fee: Stroops::new(0),
            time_bounds: None,
            memo: Memo::new_none(),
            operations: Vec::new(),
            signatures: Vec::new(),
        };
        let tx = if base_fee < MIN_BASE_FEE {
            Err(Error::TransactionFeeTooLow)
        } else {
            Ok(tx)
        };
        TransactionBuilder { tx, base_fee }
    }

    pub fn with_time_bounds(mut self, time_bounds: TimeBounds) -> TransactionBuilder {
        if let Ok(ref mut tx) = self.tx {
            *tx.time_bounds_mut() = Some(time_bounds);
        }
        self
    }

    pub fn with_memo(mut self, memo: Memo) -> TransactionBuilder {
        if let Ok(ref mut tx) = self.tx {
            *tx.memo_mut() = memo;
        }
        self
    }

    pub fn add_operation(mut self, operation: Operation) -> TransactionBuilder {
        let mut error = None;
        if let Ok(ref mut tx) = self.tx {
            let operations = tx.operations_mut();
            if operations.len() > xdr::MAX_OPS_PER_TX as usize {
                error = Some(Err(Error::TooManyOperations));
            } else {
                operations.push(operation);
            }
        }
        if let Some(error) = error {
            self.tx = error;
        }
        self
    }

    pub fn into_transaction(mut self) -> Result<Transaction> {
        let mut error = None;
        if let Ok(ref mut tx) = self.tx {
            if tx.operations().is_empty() {
                error = Some(Err(Error::MissingOperations));
            }
            let fee = self
                .base_fee
                .checked_mul(&Stroops::new(tx.operations.len() as i64))
                .ok_or(Error::TransactionFeeOverflow)?;
            *tx.fee_mut() = fee;
        }

        if let Some(error) = error {
            self.tx = error;
        }
        self.tx
    }
}

impl xdr::WriteXdr for TransactionEnvelope {
    fn write_xdr<W: Write>(&self, w: &mut xdr::Limited<W>) -> xdr::Result<()> {
        let xdr_tx = self.to_xdr().map_err(|_| xdr::Error::Invalid)?;
        xdr_tx.write_xdr(w)
    }
}

impl xdr::ReadXdr for TransactionEnvelope {
    fn read_xdr<R: Read>(r: &mut xdr::Limited<R>) -> xdr::Result<Self> {
        let xdr_result = xdr::TransactionEnvelope::read_xdr(r)?;
        Self::from_xdr(&xdr_result).map_err(|_| xdr::Error::Invalid)
    }
}

fn signatures_to_xdr(
    signatures: &[DecoratedSignature],
) -> Result<xdr::VecM<xdr::DecoratedSignature, 20>> {
    let mut xdr_signatures = Vec::new();
    for signature in signatures {
        let xdr_signature = signature.to_xdr()?;
        xdr_signatures.push(xdr_signature);
    }
    xdr_signatures.try_into().map_err(|_| Error::XdrError)
}

fn signatures_from_xdr(
    xdr_signatures: &[xdr::DecoratedSignature],
) -> Result<Vec<DecoratedSignature>> {
    let mut signatures = Vec::new();
    for xdr_signature in xdr_signatures {
        let signature = DecoratedSignature::from_xdr(xdr_signature)?;
        signatures.push(signature);
    }
    Ok(signatures)
}

#[cfg(test)]
mod tests {
    use super::Transaction;
    use crate::amount::Stroops;
    use crate::crypto::DalekKeyPair;
    use crate::memo::Memo;
    use crate::operations::Operation;
    use crate::time_bounds::TimeBounds;

    #[test]
    fn test_transaction_builder() {
        let kp = DalekKeyPair::random().unwrap();
        let tx = Transaction::builder(kp.public_key(), 123, Stroops::new(100))
            .with_memo(Memo::new_id(987))
            .with_time_bounds(TimeBounds::always_valid())
            .add_operation(Operation::new_inflation().build())
            .into_transaction()
            .unwrap();
        assert_eq!(123, *tx.sequence());
        assert_eq!(&Stroops::new(100), tx.fee());
        assert!(tx.memo().is_id());
        assert!(tx.time_bounds().is_some());
        assert_eq!(1, tx.operations().len());
    }
}
