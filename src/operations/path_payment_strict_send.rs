use crate::amount::Stroops;
use crate::asset::Asset;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::operations::Operation;
use crate::xdr;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPaymentStrictSendOperation {
    source_account: Option<MuxedAccount>,
    destination: MuxedAccount,
    send_asset: Asset,
    send_amount: Stroops,
    destination_asset: Asset,
    destination_min: Stroops,
    path: Vec<Asset>,
}

#[derive(Debug)]
pub struct PathPaymentStrictSendOperationBuilder {
    source_account: Option<MuxedAccount>,
    destination: Option<MuxedAccount>,
    send_asset: Option<Asset>,
    send_amount: Option<Stroops>,
    destination_asset: Option<Asset>,
    destination_min: Option<Stroops>,
    path: Vec<Asset>,
}

impl PathPaymentStrictSendOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    /// Retrieves the operation destination.
    pub fn destination(&self) -> &MuxedAccount {
        &self.destination
    }

    /// Retrieves a mutable reference the operation destination.
    pub fn destination_mut(&mut self) -> &mut MuxedAccount {
        &mut self.destination
    }

    /// Retrieves the operation send asset.
    pub fn send_asset(&self) -> &Asset {
        &self.send_asset
    }

    /// Retrieves a mutable reference to the operation send asset.
    pub fn send_asset_mut(&mut self) -> &mut Asset {
        &mut self.send_asset
    }

    /// Retrieves the operation send amount.
    pub fn send_amount(&self) -> &Stroops {
        &self.send_amount
    }

    /// Retrieves a mutable reference to the operation send amount.
    pub fn send_amount_mut(&mut self) -> &mut Stroops {
        &mut self.send_amount
    }

    /// Retrieves the operation destination asset.
    pub fn destination_asset(&self) -> &Asset {
        &self.destination_asset
    }

    /// Retrieves a mutable reference to the operation destination asset.
    pub fn destination_asset_mut(&mut self) -> &mut Asset {
        &mut self.destination_asset
    }

    /// Retrieves the operation destination min.
    pub fn destination_min(&self) -> &Stroops {
        &self.destination_min
    }

    /// Retrieves a mutable reference to the operation destination min.
    pub fn destination_min_mut(&mut self) -> &mut Stroops {
        &mut self.destination_min
    }

    /// Retrieves the operation path.
    pub fn path(&self) -> &Vec<Asset> {
        &self.path
    }

    /// Retrieves a mutable reference to the operation path.
    pub fn path_mut(&mut self) -> &mut Vec<Asset> {
        &mut self.path
    }

    /// Returns to the xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let destination = self.destination.to_xdr()?;
        let send_asset = self.send_asset.to_xdr()?;
        let send_amount = self.send_amount.to_xdr_int64()?;
        let dest_asset = self.destination_asset.to_xdr()?;
        let dest_min = self.destination_min.to_xdr_int64()?;
        let path_res: Result<Vec<xdr::Asset>> = self.path.iter().map(|a| a.to_xdr()).collect();
        let path = path_res?;
        let inner = xdr::PathPaymentStrictSendOp {
            destination,
            send_asset,
            send_amount,
            dest_asset,
            dest_min,
            path,
        };
        Ok(xdr::OperationBody::PathPaymentStrictSend(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::PathPaymentStrictSendOp,
    ) -> Result<PathPaymentStrictSendOperation> {
        let destination = MuxedAccount::from_xdr(&x.destination)?;
        let send_asset = Asset::from_xdr(&x.send_asset)?;
        let send_amount = Stroops::from_xdr_int64(&x.send_amount)?;
        let destination_asset = Asset::from_xdr(&x.dest_asset)?;
        let destination_min = Stroops::from_xdr_int64(&x.dest_min)?;
        let path_res: Result<Vec<Asset>> = x.path.iter().map(|a| Asset::from_xdr(&a)).collect();
        let path = path_res?;
        Ok(PathPaymentStrictSendOperation {
            source_account,
            destination,
            send_asset,
            send_amount,
            destination_asset,
            destination_min,
            path,
        })
    }
}

impl PathPaymentStrictSendOperationBuilder {
    pub fn new() -> PathPaymentStrictSendOperationBuilder {
        PathPaymentStrictSendOperationBuilder {
            source_account: None,
            destination: None,
            send_asset: None,
            send_amount: None,
            destination_asset: None,
            destination_min: None,
            path: Vec::new(),
        }
    }

    pub fn with_source_account(
        mut self,
        source: MuxedAccount,
    ) -> PathPaymentStrictSendOperationBuilder {
        self.source_account = Some(source);
        self
    }

    pub fn with_destination<A: Into<MuxedAccount>>(
        mut self,
        destination: A,
    ) -> PathPaymentStrictSendOperationBuilder {
        self.destination = Some(destination.into());
        self
    }

    pub fn with_send_asset(mut self, send_asset: Asset) -> PathPaymentStrictSendOperationBuilder {
        self.send_asset = Some(send_asset);
        self
    }

    pub fn with_send_amount<A: TryInto<Stroops>>(
        mut self,
        send_amount: A,
    ) -> Result<PathPaymentStrictSendOperationBuilder> {
        self.send_amount = Some(
            send_amount
                .try_into()
                .map_err(|_| Error::InvalidStroopsAmount)?,
        );
        Ok(self)
    }

    pub fn with_destination_asset(
        mut self,
        dest_asset: Asset,
    ) -> PathPaymentStrictSendOperationBuilder {
        self.destination_asset = Some(dest_asset);
        self
    }

    pub fn with_destination_min<A: TryInto<Stroops>>(
        mut self,
        dest_min: A,
    ) -> Result<PathPaymentStrictSendOperationBuilder> {
        self.destination_min = Some(
            dest_min
                .try_into()
                .map_err(|_| Error::InvalidStroopsAmount)?,
        );
        Ok(self)
    }

    pub fn add_asset(mut self, asset: Asset) -> PathPaymentStrictSendOperationBuilder {
        self.path.push(asset);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let destination = self
            .destination
            .ok_or_else(|| Error::InvalidOperation("missing payment destination".to_string()))?;

        let send_asset = self.send_asset.ok_or_else(|| {
            Error::InvalidOperation("missing path payment strict send send asset".to_string())
        })?;

        let send_amount = self.send_amount.ok_or_else(|| {
            Error::InvalidOperation("missing path payment strict send send amount".to_string())
        })?;

        let destination_asset = self.destination_asset.ok_or_else(|| {
            Error::InvalidOperation(
                "missing path payment strict send destination asset".to_string(),
            )
        })?;

        let destination_min = self.destination_min.ok_or_else(|| {
            Error::InvalidOperation("missing path payment strict send destination min".to_string())
        })?;

        if self.path.len() > 5 {
            return Err(Error::InvalidOperation(
                "path payment strict send path too long".to_string(),
            ));
        }

        Ok(Operation::PathPaymentStrictSend(
            PathPaymentStrictSendOperation {
                source_account: self.source_account,
                destination,
                send_asset,
                send_amount,
                destination_asset,
                destination_min,
                path: self.path,
            },
        ))
    }
}
