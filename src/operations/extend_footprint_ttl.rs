use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

/// Represents an operation to extend the footprint TTL for Soroban (smart contract) related
/// ledger entries.
///
/// Soroban introduces temporary ledger entries with a TTL (time to live) that can be extended.
/// This operation sets the TTL of all entries in the transaction's footprint to at least
/// `extend_to` (a ledger sequence number). If an entry already exceeds this value, it is
/// unaffected.
///
/// NOTE: The `ext` field in `ExtendFootprintTtlOp` is currently always the empty
/// `ExtensionPoint::V0`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendFootprintTtlOperation {
    source_account: Option<MuxedAccount>,
    extend_to: u32,
}

/// Builder for `ExtendFootprintTtlOperation`.
#[derive(Debug, Default)]
pub struct ExtendFootprintTtlOperationBuilder {
    source_account: Option<MuxedAccount>,
    extend_to: Option<u32>,
}

impl ExtendFootprintTtlOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a mutable reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the target ledger sequence number to extend TTLs to.
    pub fn extend_to(&self) -> &u32 {
        &self.extend_to
    }

    /// Retrieves a mutable reference to the target ledger sequence number.
    pub fn extend_to_mut(&mut self) -> &mut u32 {
        &mut self.extend_to
    }

    /// Returns the XDR operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inner = xdr::ExtendFootprintTtlOp {
            ext: xdr::ExtensionPoint::V0,
            extend_to: self.extend_to,
        };
        Ok(xdr::OperationBody::ExtendFootprintTtl(inner))
    }

    /// Creates from the XDR operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ExtendFootprintTtlOp,
    ) -> Result<ExtendFootprintTtlOperation> {
        Ok(ExtendFootprintTtlOperation {
            source_account,
            extend_to: x.extend_to,
        })
    }
}

impl ExtendFootprintTtlOperationBuilder {
    pub fn new() -> ExtendFootprintTtlOperationBuilder {
        Default::default()
    }

    /// Sets the source account for the operation.
    pub fn with_source_account<S>(mut self, source: S) -> ExtendFootprintTtlOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    /// Sets the ledger sequence number to extend TTLs to.
    pub fn with_extend_to(mut self, extend_to: u32) -> ExtendFootprintTtlOperationBuilder {
        self.extend_to = Some(extend_to);
        self
    }

    /// Builds the `Operation`.
    pub fn build(self) -> Result<Operation> {
        let extend_to = self.extend_to.ok_or_else(|| {
            Error::InvalidOperation("missing extend footprint ttl extend to".to_string())
        })?;

        // A TTL of 0 does not make sense (ledger sequence numbers start from 1 and
        // a value of 0 would never extend anything). Enforce > 0 to catch obvious mistakes.
        if extend_to == 0 {
            return Err(Error::InvalidOperation(
                "extend footprint ttl extend_to must be greater than zero".to_string(),
            ));
        }

        Ok(Operation::ExtendFootprintTtl(ExtendFootprintTtlOperation {
            source_account: self.source_account,
            extend_to,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_extend_footprint_ttl_roundtrip() {
        let op = ExtendFootprintTtlOperationBuilder::new()
            .with_extend_to(12345)
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let decoded = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, decoded);
        assert!(decoded.source_account().is_none());
        if let Operation::ExtendFootprintTtl(inner) = &decoded {
            assert_eq!(*inner.extend_to(), 12345);
        } else {
            panic!("expected ExtendFootprintTtl operation");
        }
    }

    #[test]
    fn test_extend_footprint_ttl_with_source_account() {
        let source = keypair0().public_key();
        let op = ExtendFootprintTtlOperationBuilder::new()
            .with_source_account(source)
            .with_extend_to(999_999)
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let decoded = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, decoded);
        let inner = match &decoded {
            Operation::ExtendFootprintTtl(op) => op,
            _ => panic!("expected ExtendFootprintTtl operation"),
        };
        assert_eq!(*inner.extend_to(), 999_999);
        assert!(inner.source_account().is_some());
    }
}
