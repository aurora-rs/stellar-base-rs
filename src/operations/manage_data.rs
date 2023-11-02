use crate::account::DataValue;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManageDataOperation {
    source_account: Option<MuxedAccount>,
    data_name: String,
    data_value: Option<DataValue>,
}

#[derive(Debug, Default)]
pub struct ManageDataOperationBuilder {
    source_account: Option<MuxedAccount>,
    data_name: Option<String>,
    data_value: Option<DataValue>,
}

impl ManageDataOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation data name.
    pub fn data_name(&self) -> &str {
        &self.data_name
    }

    /// Retrieves a mutable reference to the operation data name.
    pub fn data_name_mut(&mut self) -> &mut str {
        &mut self.data_name
    }

    /// Retrieves the operation data value.
    pub fn data_value(&self) -> &Option<DataValue> {
        &self.data_value
    }

    /// Retrieves a mutable reference to the operation data value.
    pub fn data_value_mut(&mut self) -> &mut Option<DataValue> {
        &mut self.data_value
    }

    /// Returns the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let data_name = xdr::String64::new(self.data_name.to_string());
        let data_value = self.data_value.as_ref().map(|d| d.to_xdr()).transpose()?;
        let inner = xdr::ManageDataOp {
            data_name,
            data_value,
        };
        Ok(xdr::OperationBody::ManageData(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ManageDataOp,
    ) -> Result<ManageDataOperation> {
        let data_name = x.data_name.value.to_string();
        let data_value = x
            .data_value
            .as_ref()
            .map(DataValue::from_xdr)
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
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> ManageDataOperationBuilder
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
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

#[cfg(test)]
mod tests {
    use crate::account::DataValue;

    use crate::network::Network;
    use crate::operations::tests::*;
    use crate::operations::Operation;
    use crate::transaction::{Transaction, TransactionEnvelope, MIN_BASE_FEE};
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_manage_data() {
        let kp = keypair0();
        let value = DataValue::from_slice("value value".as_bytes()).unwrap();
        let op = Operation::new_manage_data()
            .with_data_name("TEST TEST".to_string())
            .with_data_value(Some(value))
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAAAAAAKAAAACVRFU1QgVEVTVAAAAAAAAAEAAAALdmFsdWUgdmFsdWUAAAAAAAAAAAHqLnLFAAAAQLxeb1DkXDTXi/rOffnHpyxuJhl8vN/GDMKLtxFFTGn5b99FNHmWUyUoxb4KTE9bBguIe33SEQ/npj32f2vt/gY=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }

    #[test]
    fn test_manage_data_with_source_account() {
        let kp = keypair0();
        let kp1 = keypair1();
        let value = DataValue::from_slice("value value".as_bytes()).unwrap();
        let op = Operation::new_manage_data()
            .with_source_account(kp1.public_key())
            .with_data_name("TEST TEST".to_string())
            .with_data_value(Some(value))
            .build()
            .unwrap();

        let mut tx = Transaction::builder(kp.public_key(), 3556091187167235, MIN_BASE_FEE)
            .add_operation(op)
            .into_transaction()
            .unwrap();
        tx.sign(kp.as_ref(), &Network::new_test()).unwrap();
        let envelope = tx.to_envelope();
        let xdr = envelope.xdr_base64().unwrap();
        let expected = "AAAAAgAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAAGQADKI/AAAAAwAAAAAAAAAAAAAAAQAAAAEAAAAAJcrx2g/Hbs/ohF5CVFG7B5JJSJR+OqDKzDGK7dKHZH4AAAAKAAAACVRFU1QgVEVTVAAAAAAAAAEAAAALdmFsdWUgdmFsdWUAAAAAAAAAAAHqLnLFAAAAQBQKnwjKQ1RbYg0rk7G9VV1jHwM29YEp1EoOug960nTVWga6aFmPlQ0mDDudEsbSMq+9G8eYX5mcu9EHTjZUBQI=";
        assert_eq!(expected, xdr);
        let back = TransactionEnvelope::from_xdr_base64(&xdr).unwrap();
        assert_eq!(envelope, back);
    }
}
