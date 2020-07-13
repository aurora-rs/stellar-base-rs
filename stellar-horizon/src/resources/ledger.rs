use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ledger {
    id: String,
    paging_token: String,
    hash: String,
    sequence: i64,
    successful_transaction_count: i64,
    failed_transaction_count: i64,
    operation_count: i64,
    closed_at: String,
    total_coins: String,
    fee_pool: String,
    base_fee_in_stroops: i64,
    base_reserve_in_stroops: i64,
    max_tx_set_size: i64,
    protocol_version: i64,
    header_xdr: String,
}
