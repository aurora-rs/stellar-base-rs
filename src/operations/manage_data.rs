use crate::account::DataValue;
use crate::amount::{Price, Stroops};
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManageDataOperation {
    source_account: Option<MuxedAccount>,
    data_name: String,
    data_value: Option<DataValue>,
}

#[derive(Debug)]
pub struct ManageDataOperationBuilder {
    source_account: Option<MuxedAccount>,
    data_name: Option<String>,
    data_value: Option<DataValue>,
}

impl ManageDataOperation {
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn data_name(&self) -> &str {
        &self.data_name
    }

    pub fn data_value(&self) -> &Option<DataValue> {
        &self.data_value
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let data_name = xdr::String64::new(self.data_name.to_string());
        let data_value = self.data_value.as_ref().map(|d| d.to_xdr()).transpose()?;
        let inner = xdr::ManageDataOp {
            data_name,
            data_value,
        };
        Ok(xdr::OperationBody::ManageData(inner))
    }

    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ManageDataOp,
    ) -> Result<ManageDataOperation> {
        let data_name = x.data_name.value.to_string();
        let data_value = x
            .data_value
            .as_ref()
            .map(|d| DataValue::from_xdr(d))
            .transpose()?;

        Ok(ManageDataOperation {
            source_account,
            data_name,
            data_value,
        })
    }
}

impl ManageDataOperationBuilder {
    pub fn new() -> ManageDataOperationBuilder {
        ManageDataOperationBuilder {
            source_account: None,
            data_name: None,
            data_value: None,
        }
    }

    pub fn with_source_account(mut self, source: MuxedAccount) -> ManageDataOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_data_name(mut self, name: String) -> ManageDataOperationBuilder {
        self.data_name = Some(name);
        self
    }

    pub fn with_data_value(mut self, value: Option<DataValue>) -> ManageDataOperationBuilder {
        self.data_value = value;
        self
    }

    pub fn build(self) -> Result<Operation> {
        let data_name = self
            .data_name
            .ok_or_else(|| Error::InvalidOperation("missing manage data data name".to_string()))?;

        Ok(Operation::ManageData(ManageDataOperation {
            source_account: self.source_account,
            data_name,
            data_value: self.data_value,
        }))
    }
}
