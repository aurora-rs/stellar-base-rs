use crate::resources::{Asset, Link, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trade {
    #[serde(rename = "_links")]
    links: TradeLinks,
    id: String,
    paging_token: String,
    ledger_close_time: DateTime<Utc>,
    offer_id: String,
    base_offer_id: String,
    base_account: String,
    base_amount: String,
    #[serde(flatten, with = "BaseAsset")]
    base_asset: Asset,
    counter_offer_id: String,
    counter_account: String,
    counter_amount: String,
    #[serde(flatten, with = "CounterAsset")]
    couter_asset: Asset,
    base_is_seller: bool,
    price: Option<Price>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeEffect {
    #[serde(rename = "_links")]
    links: TradeEffectLinks,
    id: String,
    paging_token: String,
    offer_id: String,
    seller: String,
    sold_amount: String,
    #[serde(flatten, with = "SoldAsset")]
    sold_asset: Asset,
    buyer: String,
    bought_amount: String,
    #[serde(flatten, with = "BoughtAsset")]
    bought_asset: Asset,
    created_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeAggregation {
    timestamp: String,
    trade_count: String,
    base_volume: String,
    counter_volume: String,
    #[serde(rename = "avg")]
    average: String,
    #[serde(rename = "high_r")]
    high_ratio: Price,
    high: String,
    #[serde(rename = "low_r")]
    low_ratio: Price,
    low: String,
    #[serde(rename = "open_r")]
    open_ratio: Price,
    open: String,
    #[serde(rename = "close_r")]
    close_ration: Price,
    close: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeLinks {
    #[serde(rename = "self")]
    self_: Link,
    base: Link,
    counter: Link,
    operation: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeEffectLinks {
    #[serde(rename = "self")]
    self_: Link,
    seller: Link,
    buyer: Link,
    operation: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct BaseAsset {
    #[serde(rename = "base_asset_type")]
    asset_type: String,
    #[serde(rename = "base_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "base_asset_issuer")]
    asset_isuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct CounterAsset {
    #[serde(rename = "counter_asset_type")]
    asset_type: String,
    #[serde(rename = "counter_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "counter_asset_issuer")]
    asset_isuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct SoldAsset {
    #[serde(rename = "sold_asset_type")]
    asset_type: String,
    #[serde(rename = "sold_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "sold_asset_issuer")]
    asset_isuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct BoughtAsset {
    #[serde(rename = "bought_asset_type")]
    asset_type: String,
    #[serde(rename = "bought_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "bought_asset_issuer")]
    asset_isuer: Option<String>,
}
