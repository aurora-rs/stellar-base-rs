use crate::resources::Link;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ledger {
    #[serde(rename = "_links")]
    links: LedgerLinks,
    id: String,
    paging_token: String,
    hash: String,
    #[serde(rename = "prev_hash", skip_serializing_if = "Option::is_none")]
    previous_hash: Option<String>,
    sequence: i32,
    successful_transaction_count: i32,
    failed_transaction_count: Option<i32>,
    operation_count: i32,
    #[serde(rename = "tx_set_operation_count")]
    transaction_set_operation_count: Option<i32>,
    closed_at: DateTime<Utc>,
    total_coins: String,
    fee_pool: String,
    base_fee_in_stroops: i32,
    base_reserve_in_stroops: i32,
    #[serde(rename = "max_tx_set_size")]
    max_transaction_set_size: i32,
    protocol_version: i32,
    header_xdr: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeDistribution {
    max: String,
    min: String,
    mode: String,
    p10: String,
    p20: String,
    p30: String,
    p40: String,
    p50: String,
    p60: String,
    p70: String,
    p80: String,
    p90: String,
    p95: String,
    p99: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeStats {
    last_ledger: String,
    last_ledger_base_fee: String,
    ledger_capacity_usage: String,
    fee_charged: FeeDistribution,
    max_fee: FeeDistribution,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LedgerLinks {
    #[serde(rename = "self")]
    self_: Link,
    transactions: Link,
    operations: Link,
    payments: Link,
    effects: Link,
}
