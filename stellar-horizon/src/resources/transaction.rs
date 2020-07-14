use crate::resources::{Asset, Link, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(rename = "_links")]
    links: TransactionLinks,
    id: String,
    paging_token: String,
    successful: bool,
    hash: String,
    ledger: i32,
    created_at: DateTime<Utc>,
    source_account: String,
    source_account_sequence: String,
    fee_account: String,
    fee_charged: String,
    max_fee: String,
    operation_count: i32,
    envelope_xdr: String,
    result_xdr: String,
    result_meta_xdr: String,
    fee_meta_xdr: String,
    memo_type: String,
    memo_bytes: Option<String>,
    memo: Option<String>,
    signatures: Vec<String>,
    valid_after: Option<String>,
    valid_before: Option<String>,
    fee_bump_transaction: Option<FeeBumpTransaction>,
    inner_transaction: Option<InnerTransaction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeBumpTransaction {
    hash: String,
    signatures: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InnerTransaction {
    hash: String,
    signatures: Vec<String>,
    max_fee: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionResultCodes {
    transaction: String,
    operations: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionLinks {
    #[serde(rename = "self")]
    self_: Link,
    account: Link,
    ledger: Link,
    operations: Link,
    effects: Link,
    precedes: Link,
    succeeds: Link,
    transaction: Link,
}
