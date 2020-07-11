use crate::amount::Stroops;
use crate::asset::Asset;
use crate::crypto::{MuxedAccount, PublicKey};
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPaymentStrictReceiveOperation {
    source_account: Option<MuxedAccount>,
    destination: MuxedAccount,
    send_asset: Asset,
    send_max: Stroops,
    destination_asset: Asset,
    destination_amount: Stroops,
    path: Vec<Asset>,
}

#[derive(Debug)]
pub struct PathPaymentStrictReceiveOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<MuxedAccount>,
    send_asset: Option<Asset>,
    send_max: Option<Stroops>,
    destination_asset: Option<Asset>,
    destination_amount: Option<Stroops>,
    path: Vec<Asset>,
}

impl PathPaymentStrictReceiveOperation {
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    pub fn destination(&self) -> &MuxedAccount {
        &self.destination
    }

    pub fn send_asset(&self) -> &Asset {
        &self.send_asset
    }

    pub fn send_max(&self) -> &Stroops {
        &self.send_max
    }

    pub fn destination_asset(&self) -> &Asset {
        &self.destination_asset
    }

    pub fn destination_amount(&self) -> &Stroops {
        &self.destination_amount
    }

    pub fn path(&self) -> &Vec<Asset> {
        &self.path
    }

    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let destination = self.destination.to_xdr()?;
        let send_asset = self.send_asset.to_xdr()?;
        let send_max = self.send_max.to_xdr_int64()?;
        let dest_asset = self.destination_asset.to_xdr()?;
        let dest_amount = self.destination_amount.to_xdr_int64()?;
        let path_res: Result<Vec<xdr::Asset>> = self.path.iter().map(|a| a.to_xdr()).collect();
        let path = path_res?;
        let inner = xdr::PathPaymentStrictReceiveOp {
            destination,
            send_asset,
            send_max,
            dest_asset,
            dest_amount,
            path,
        };
        Ok(xdr::OperationBody::PathPaymentStrictReceive(inner))
    }

    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::PathPaymentStrictReceiveOp,
    ) -> Result<PathPaymentStrictReceiveOperation> {
        let destination = MuxedAccount::from_xdr(&x.destination)?;
        let send_asset = Asset::from_xdr(&x.send_asset)?;
        let send_max = Stroops::from_xdr_int64(&x.send_max)?;
        let destination_asset = Asset::from_xdr(&x.dest_asset)?;
        let destination_amount = Stroops::from_xdr_int64(&x.dest_amount)?;
        let path_res: Result<Vec<Asset>> = x.path.iter().map(|a| Asset::from_xdr(&a)).collect();
        let path = path_res?;
        Ok(PathPaymentStrictReceiveOperation {
            source_account,
            destination,
            send_asset,
            send_max,
            destination_asset,
            destination_amount,
            path,
        })
    }
}

impl PathPaymentStrictReceiveOperationBuilder {
    pub fn new() -> PathPaymentStrictReceiveOperationBuilder {
        PathPaymentStrictReceiveOperationBuilder {
            source_account: None,
            destination: None,
            send_asset: None,
            send_max: None,
            destination_asset: None,
            destination_amount: None,
            path: Vec::new(),
        }
    }

    pub fn with_source_account(
        mut self,
        source: MuxedAccount,
    ) -> PathPaymentStrictReceiveOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let destination = self
            .destination
            .ok_or_else(|| Error::InvalidOperation("missing payment destination".to_string()))?;

        let send_asset = self.send_asset.ok_or_else(|| {
            Error::InvalidOperation("missing path payment strict send send asset".to_string())
        })?;

        let send_max = self.send_max.ok_or_else(|| {
            Error::InvalidOperation("missing path payment strict send send max".to_string())
        })?;

        let destination_asset = self.destination_asset.ok_or_else(|| {
            Error::InvalidOperation(
                "missing path payment strict send destination asset".to_string(),
            )
        })?;

        let destination_amount = self.destination_amount.ok_or_else(|| {
            Error::InvalidOperation(
                "missing path payment strict send destination amount".to_string(),
            )
        })?;

        if self.path.len() > 5 {
            return Err(Error::InvalidOperation(
                "path payment strict send path too long".to_string(),
            ));
        }

        Ok(Operation::PathPaymentStrictReceive(
            PathPaymentStrictReceiveOperation {
                source_account: self.source_account,
                destination,
                send_asset,
                send_max,
                destination_asset,
                destination_amount,
                path: self.path,
            },
        ))
    }
}
