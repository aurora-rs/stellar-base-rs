use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use url::Url;

#[derive(Debug, Clone)]
pub struct SingleLedgerRequest {
    ledger_sequence: u64,
}

#[derive(Debug, Clone)]
pub struct AllLedgersRequest {
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

pub fn single(ledger_sequence: u64) -> SingleLedgerRequest {
    SingleLedgerRequest { ledger_sequence }
}

pub fn all() -> AllLedgersRequest {
    AllLedgersRequest {
        limit: None,
        cursor: None,
        order: None,
    }
}

impl Request for SingleLedgerRequest {
    type Response = resources::Ledger;

    fn uri(&self, host: &Url) -> Result<Url> {
        let path = format!("/ledgers/{}", self.ledger_sequence);
        Ok(host.join(&path)?)
    }
}

impl Request for AllLedgersRequest {
    type Response = Page<resources::Ledger>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join("/ledgers")?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllLedgersRequest);

impl StreamRequest for AllLedgersRequest {
    type Resource = resources::Ledger;
}
