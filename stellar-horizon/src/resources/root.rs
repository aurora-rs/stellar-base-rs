use crate::resources::Link;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "_links")]
    links: RootLinks,
    horizon_version: String,
    core_version: String,
    ingest_latest_ledger: u32,
    history_latest_ledger: i32,
    history_elder_ledger: i32,
    core_latest_ledger: i32,
    network_passphrase: String,
    current_protocol_version: i32,
    core_supported_protocol_version: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootLinks {
    account: Link,
    accounts: Option<Link>,
    account_transactions: Link,
    assets: Link,
    effects: Link,
    fee_stats: Link,
    friendbot: Option<Link>,
    ledger: Link,
    ledgers: Link,
    offer: Option<Link>,
    offers: Option<Link>,
    operation: Link,
    operations: Link,
    order_book: Link,
    payments: Link,
    #[serde(rename = "self")]
    self_: Link,
    strict_receive_paths: Option<Link>,
    strict_send_paths: Option<Link>,
    trade_aggregations: Link,
    trades: Link,
    transaction: Link,
    transactions: Link,
}
