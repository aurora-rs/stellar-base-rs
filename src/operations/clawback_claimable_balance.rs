use crate::claim::ClaimableBalanceId;
use crate::crypto::MuxedAccount;
use crate::error::{Error, Result};
use crate::{xdr, Operation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClawbackClaimableBalanceOperation {
    source_account: Option<MuxedAccount>,
    balance_id: ClaimableBalanceId,
}

#[derive(Debug, Default)]
pub struct ClawbackClaimableBalanceOperationBuilder {
    source_account: Option<MuxedAccount>,
    balance_id: Option<ClaimableBalanceId>,
}

impl ClawbackClaimableBalanceOperation {
    /// Retrieves the operation source account.
    pub fn source_account(&self) -> &Option<MuxedAccount> {
        &self.source_account
    }

    /// Retrieves a reference to the operation source account.
    pub fn source_account_mut(&mut self) -> &mut Option<MuxedAccount> {
        &mut self.source_account
    }

    pub fn balance_id(&self) -> &ClaimableBalanceId {
        &self.balance_id
    }

    pub fn balance_id_mut(&mut self) -> &mut ClaimableBalanceId {
        &mut self.balance_id
    }

    /// Returns tho xdr operation body.
    pub fn to_xdr_operation_body(&self) -> Result<xdr::OperationBody> {
        let inner = xdr::ClawbackClaimableBalanceOp {
            balance_id: self.balance_id.to_xdr(),
        };
        Ok(xdr::OperationBody::ClawbackClaimableBalance(inner))
    }

    /// Creates from the xdr operation body.
    pub fn from_xdr_operation_body(
        source_account: Option<MuxedAccount>,
        x: &xdr::ClawbackClaimableBalanceOp,
    ) -> Result<Self> {
        Ok(Self {
            source_account,
            balance_id: ClaimableBalanceId::from_xdr(&x.balance_id)?,
        })
    }
}

impl ClawbackClaimableBalanceOperationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_source_account<S>(mut self, source: S) -> Self
    where
        S: Into<MuxedAccount>,
    {
        self.source_account = Some(source.into());
        self
    }

    pub fn with_balance_id(mut self, balance_id: ClaimableBalanceId) -> Self {
        self.balance_id = Some(balance_id);
        self
    }

    pub fn build(self) -> Result<Operation> {
        let balance_id = self.balance_id.ok_or_else(|| {
            Error::InvalidOperation(
                "missing balance id for clawback claimable balance operation".to_string(),
            )
        })?;

        Ok(Operation::ClawbackClaimableBalance(
            ClawbackClaimableBalanceOperation {
                source_account: self.source_account,
                balance_id,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::claim::ClaimableBalanceId;
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use crate::{KeyPair, Operation};

    fn keypair0() -> KeyPair {
        // GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3
        KeyPair::from_secret_seed("SBPQUZ6G4FZNWFHKUWC5BEYWF6R52E3SEP7R3GWYSM2XTKGF5LNTWW4R")
            .unwrap()
    }

    #[test]
    fn test_clawback_claimable_balance() {
        let balance_id = ClaimableBalanceId::new(vec![
            0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD,
            0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
            0xDE, 0xAD, 0xBE, 0xEF,
        ])
        .unwrap();

        let op = Operation::new_clawback_claimable_balance()
            .with_balance_id(balance_id)
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let expected = "AAAAAAAAABQAAAAA3q2+796tvu/erb7v3q2+796tvu/erb7v3q2+796tvu8=";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, back);
    }

    #[test]
    fn test_clawback_claimable_balance_with_source_account() {
        let source_account = keypair0();

        let balance_id = ClaimableBalanceId::new(vec![
            0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD,
            0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
            0xDE, 0xAD, 0xBE, 0xEF,
        ])
        .unwrap();

        let op = Operation::new_clawback_claimable_balance()
            .with_source_account(source_account.public_key().clone())
            .with_balance_id(balance_id)
            .build()
            .unwrap();

        let encoded = op.xdr_base64().unwrap();
        let expected = "AAAAAQAAAADg3G3hclysZlFitS+s5zWyiiJD5B0STWy5LXCj6i5yxQAAABQAAAAA3q2+796tvu/erb7v3q2+796tvu/erb7v3q2+796tvu8=";
        assert_eq!(expected, encoded);
        let back = Operation::from_xdr_base64(&encoded).unwrap();
        assert_eq!(op, back);
    }
}
