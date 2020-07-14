use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::crypto::PublicKey;
use stellar_base::transaction::TransactionEnvelope;
use stellar_base::xdr::XDRSerialize;
use url::Url;

pub fn all() -> AllTransactionsRequest {
    AllTransactionsRequest {
        limit: None,
        cursor: None,
        order: None,
    }
}

pub fn single<S: Into<String>>(id: S) -> SingleTransactionRequest {
    SingleTransactionRequest { id: id.into() }
}

pub fn submit(tx: &TransactionEnvelope) -> Result<SubmitTransactionRequest> {
    let xdr = tx.xdr_base64()?;
    Ok(SubmitTransactionRequest { xdr })
}

pub fn for_account(account: &PublicKey) -> TransactionsForAccountRequest {
    TransactionsForAccountRequest {
        account_id: account.account_id(),
        limit: None,
        cursor: None,
        order: None,
    }
}

pub fn for_ledger(ledger: u32) -> TransactionsForLedgerRequest {
    TransactionsForLedgerRequest {
        ledger,
        limit: None,
        cursor: None,
        order: None,
    }
}

#[derive(Debug, Clone)]
pub struct AllTransactionsRequest {
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

#[derive(Debug, Clone)]
pub struct SingleTransactionRequest {
    id: String,
}

#[derive(Debug, Clone)]
pub struct SubmitTransactionRequest {
    xdr: String,
}

#[derive(Debug, Clone)]
pub struct TransactionsForAccountRequest {
    account_id: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

#[derive(Debug, Clone)]
pub struct TransactionsForLedgerRequest {
    ledger: u32,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for AllTransactionsRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join("/transactions")?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllTransactionsRequest);

impl StreamRequest for AllTransactionsRequest {
    type Resource = resources::Transaction;
}

impl Request for SingleTransactionRequest {
    type Response = resources::Transaction;

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.join(&format!("/transactions/{}", self.id))?)
    }
}

impl Request for SubmitTransactionRequest {
    type Response = resources::Transaction;

    fn is_post(&self) -> bool {
        true
    }

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.join("/transactions")?)
    }
}

impl Request for TransactionsForAccountRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join(&format!("/accounts/{}/transactions", self.account_id))?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(TransactionsForAccountRequest);

impl Request for TransactionsForLedgerRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join(&format!("/ledgers/{}/transactions", self.ledger))?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(TransactionsForLedgerRequest);
