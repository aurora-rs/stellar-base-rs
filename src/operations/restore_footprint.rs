use crate::crypto::MuxedAccount;
use crate::error::Result;
use crate::operations::Operation;
use crate::xdr;

/// Represents an operation to restore the Soroban footprint.
///
/// Soroban transactions include a footprint describing which ledger entries they
/// read and/or write. Some ledger entries used by smart contracts may have a TTL
/// (time to live) in the eviction queue. This operation restores any evicted
/// entries referenced in the transaction's footprint back into the ledger.
///
/// The XDR struct `RestoreFootprintOp` currently contains only an `ExtensionPoint`
/// which is always the `V0` (empty) variant for now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreFootprintOperation {
    source_account: Option<MuxedAccount>,
}

#[derive(Debug, Default)]
pub struct RestoreFootprintOperationBuilder {
    source_account: Option<MuxedAccount>,
}

impl RestoreFootprintOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a mutable reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Returns the XDR operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inner = xdr::RestoreFootprintOp {
            ext: xdr::ExtensionPoint::V0,
        };
        Ok(xdr::OperationBody::RestoreFootprint(inner))
    }

    /// Creates from the XDR operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        _x: &xdr::RestoreFootprintOp,
    ) -> Result<RestoreFootprintOperation> {
        Ok(RestoreFootprintOperation { source_account })
    }
}

impl RestoreFootprintOperationBuilder {
    pub fn new() -> RestoreFootprintOperationBuilder {
        Default::default()
    }

    /// Sets the operation source account.
    pub fn with_source_account<S>(mut self, source: S) -> RestoreFootprintOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    /// Builds the operation.
    pub fn build(self) -> Operation {
        Operation::RestoreFootprint(RestoreFootprintOperation {
            source_account: self.source_account,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_restore_footprint_roundtrip() {
        let op = RestoreFootprintOperationBuilder::new().build();
        let encoded = op.xdr_base64().unwrap();
        let decoded = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, decoded);
        if let Operation::RestoreFootprint(inner) = decoded {
            assert!(inner.source_account().is_none());
        } else {
            panic!("expected RestoreFootprint operation");
        }
    }

    #[test]
    fn test_restore_footprint_with_source_account() {
        let source = keypair0().public_key();
        let op = RestoreFootprintOperationBuilder::new()
            .with_source_account(source)
            .build();
        let encoded = op.xdr_base64().unwrap();
        let decoded = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, decoded);
        match decoded {
            Operation::RestoreFootprint(inner) => {
                assert!(inner.source_account().is_some());
            }
            _ => panic!("expected RestoreFootprint operation"),
        }
    }
}
