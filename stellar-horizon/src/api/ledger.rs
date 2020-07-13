use crate::api::Page;
use crate::client::{Request, StreamRequest};
use crate::error::Result;
use crate::resources;

pub struct SingleLedgerRequest {
    ledger_sequence: u64,
}

pub struct AllLedgersRequest {}

pub fn single(ledger_sequence: u64) -> SingleLedgerRequest {
    SingleLedgerRequest { ledger_sequence }
}

pub fn all() -> AllLedgersRequest {
    AllLedgersRequest {}
}

impl Request for SingleLedgerRequest {
    type Response = resources::Ledger;

    fn uri(&self, host: &str) -> Result<String> {
        Ok(format!("{}/ledgers/{}", host, self.ledger_sequence))
    }
}

impl Request for AllLedgersRequest {
    type Response = Page<resources::Ledger>;

    fn uri(&self, host: &str) -> Result<String> {
        Ok(format!("{}/ledgers", host))
    }
}

impl StreamRequest for AllLedgersRequest {
    type Resource = resources::Ledger;
}
