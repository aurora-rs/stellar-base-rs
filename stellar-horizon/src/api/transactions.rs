use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use url::Url;

#[derive(Debug, Clone)]
pub struct AllTransactionsRequest {}

#[derive(Debug, Clone)]
pub struct SingleTransactionRequest {}

#[derive(Debug, Clone)]
pub struct SubmitTransactionRequest {}

#[derive(Debug, Clone)]
pub struct TransactionsForAccountRequest {}

#[derive(Debug, Clone)]
pub struct TransactionsForLedgerRequest {}
