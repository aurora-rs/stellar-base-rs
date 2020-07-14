use crate::resources::{Asset, Link};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename = "_links")]
    links: AccountLinks,
    id: String,
    account_id: String,
    sequence: String,
    subentry_count: i32,
    inflation_destination: Option<String>,
    home_domain: String,
    last_modified_ledger: u32,
    last_modified_time: Option<DateTime<Utc>>,
    thresholds: AccountThresholds,
    flags: AccountFlags,
    balances: Vec<Balance>,
    signers: Vec<Signer>,
    data: Map<String, String>,
    paging_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountLinks {
    #[serde(rename = "self")]
    self_: Link,
    transactions: Link,
    operations: Link,
    payments: Link,
    effects: Link,
    offers: Link,
    trades: Link,
    data: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountThresholds {
    low_threshold: u8,
    medium_threshold: u8,
    high_threshold: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountFlags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Balance {
    balance: String,
    limit: Option<String>,
    buying_liabilities: String,
    selling_liabilities: String,
    last_modified_ledger: Option<u32>,
    is_authorized: Option<bool>,
    is_authorized_to_maintain_liabilities: Option<bool>,
    #[serde(flatten)]
    asset: Asset,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountSigner {
    #[serde(rename = "_links")]
    links: AccountSignerLinks,
    id: String,
    account_id: String,
    paging_token: String,
    signer: Signer,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountData {
    value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountSignerLinks {
    account: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signer {
    weight: i32,
    key: String,
    #[serde(rename = "type")]
    type_: String,
}
