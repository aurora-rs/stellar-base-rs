use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

/// Represents an operation invoking a Soroban host function.
///
/// Soroban smart contracts are accessed through host functions that are executed
/// atomically within the context of the transaction. This operation carries:
/// - The host function (one of:
///     * InvokeContract
///     * CreateContract
///     * UploadContractWasm
///     * CreateContractV2
///   )
/// - Authorization entries proving the required signatures / invocation authorizations
///
/// The XDR struct backing this is:
/// ```text
/// struct InvokeHostFunctionOp {
///     HostFunction hostFunction;
///     SorobanAuthorizationEntry auth<>;
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeHostFunctionOperation {
    source_account: Option<MuxedAccount>,
    host_function: xdr::HostFunction,
    auth: Vec<xdr::SorobanAuthorizationEntry>,
}

/// Builder for `InvokeHostFunctionOperation`.
#[derive(Debug, Default)]
pub struct InvokeHostFunctionOperationBuilder {
    source_account: Option<MuxedAccount>,
    host_function: Option<xdr::HostFunction>,
    auth: Vec<xdr::SorobanAuthorizationEntry>,
}

impl InvokeHostFunctionOperation {
    /// Source account (if any) overriding the transaction source.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Mutable reference to the source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// The host function to invoke.
    pub fn host_function(&self) -> &xdr::HostFunction {
        &self.host_function
    }

    /// Mutable reference to the host function.
    pub fn host_function_mut(&mut self) -> &mut xdr::HostFunction {
        &mut self.host_function
    }

    /// Authorization entries.
    pub fn auth(&self) -> &[xdr::SorobanAuthorizationEntry] {
        &self.auth
    }

    /// Mutable authorization entries.
    pub fn auth_mut(&mut self) -> &mut Vec<xdr::SorobanAuthorizationEntry> {
        &mut self.auth
    }

    /// Returns the XDR operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let auth_xdr = self.auth.clone().try_into().map_err(|_| Error::XdrError)?;
        let inner = xdr::InvokeHostFunctionOp {
            host_function: self.host_function.clone(),
            auth: auth_xdr,
        };
        Ok(xdr::OperationBody::InvokeHostFunction(inner))
    }

    /// Creates from the XDR operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::InvokeHostFunctionOp,
    ) -> Result<InvokeHostFunctionOperation> {
        let auth: Vec<xdr::SorobanAuthorizationEntry> = x.auth.iter().cloned().collect();
        Ok(InvokeHostFunctionOperation {
            source_account,
            host_function: x.host_function.clone(),
            auth,
        })
    }
}

impl InvokeHostFunctionOperationBuilder {
    /// New builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the source account.
    pub fn with_source_account<S>(mut self, source: S) -> Self
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    /// Sets the host function.
    pub fn with_host_function(mut self, hf: xdr::HostFunction) -> Self {
        self.host_function = Some(hf);
        self
    }

    /// Replaces the authorization entries.
    pub fn with_auth(mut self, auth: Vec<xdr::SorobanAuthorizationEntry>) -> Self {
        self.auth = auth;
        self
    }

    /// Adds a single authorization entry.
    pub fn add_auth_entry(mut self, entry: xdr::SorobanAuthorizationEntry) -> Self {
        self.auth.push(entry);
        self
    }

    /// Convenience: host function InvokeContract.
    pub fn with_invoke_contract(
        mut self,
        contract_address: xdr::ScAddress,
        function_name: xdr::ScSymbol,
        args: Vec<xdr::ScVal>,
    ) -> Self {
        let args_xdr = args.try_into().unwrap_or_else(|_| xdr::VecM::default());
        let invoke_args = xdr::InvokeContractArgs {
            contract_address,
            function_name,
            args: args_xdr,
        };
        self.host_function = Some(xdr::HostFunction::InvokeContract(invoke_args));
        self
    }

    /// Convenience: host function UploadContractWasm.
    pub fn with_upload_wasm(mut self, wasm: xdr::BytesM) -> Self {
        self.host_function = Some(xdr::HostFunction::UploadContractWasm(wasm));
        self
    }

    /// Convenience: host function CreateContract.
    pub fn with_create_contract(
        mut self,
        contract_id_preimage: xdr::ContractIdPreimage,
        executable: xdr::ContractExecutable,
    ) -> Self {
        let args = xdr::CreateContractArgs {
            contract_id_preimage,
            executable,
        };
        self.host_function = Some(xdr::HostFunction::CreateContract(args));
        self
    }

    /// Convenience: host function CreateContractV2.
    pub fn with_create_contract_v2(
        mut self,
        contract_id_preimage: xdr::ContractIdPreimage,
        executable: xdr::ContractExecutable,
        constructor_args: Vec<xdr::ScVal>,
    ) -> Self {
        let ctor_args_xdr = constructor_args
            .try_into()
            .unwrap_or_else(|_| xdr::VecM::default());
        let args = xdr::CreateContractArgsV2 {
            contract_id_preimage,
            executable,
            constructor_args: ctor_args_xdr,
        };
        self.host_function = Some(xdr::HostFunction::CreateContractV2(args));
        self
    }

    /// Builds the operation.
    pub fn build(self) -> Result<Operation> {
        let host_function = self.host_function.ok_or_else(|| {
            Error::InvalidOperation("missing host function for invoke host function".to_string())
        })?;

        // Validate number of auth entries vs some arbitrary large limit used when constructing VecM.
        if self.auth.len() > 10_000 {
            return Err(Error::InvalidOperation(
                "too many authorization entries for invoke host function".to_string(),
            ));
        }

        Ok(Operation::InvokeHostFunction(InvokeHostFunctionOperation {
            source_account: self.source_account,
            host_function,
            auth: self.auth,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::Operation;

    use crate::xdr::{ScAddress, ScSymbol, XDRDeserialize, XDRSerialize};

    #[test]
    fn test_invoke_host_function_invoke_contract_roundtrip() {
        // Build a simple InvokeContract host function.
        let contract_hash: [u8; 32] = [1u8; 32];
        let addr = ScAddress::Contract(contract_hash.into());
        // For simplicity, construct a symbol from a short ascii name. ScSymbol in current XDR is a
        // length-limited opaque or string-like type; we rely on From<String> or TryFrom<Vec<u8>>.
        let symbol: ScSymbol = "ping".to_string().try_into().unwrap();

        let op = InvokeHostFunctionOperationBuilder::new()
            .with_invoke_contract(addr, symbol, vec![])
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let decoded = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, decoded);
        if let Operation::InvokeHostFunction(inner) = decoded {
            assert!(matches!(
                inner.host_function(),
                xdr::HostFunction::InvokeContract(_)
            ));
        } else {
            panic!("expected invoke host function operation");
        }
    }
}
