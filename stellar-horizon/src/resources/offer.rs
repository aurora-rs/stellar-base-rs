use crate::resources::{Asset, Link, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Offer {
    #[serde(rename = "_links")]
    links: OfferLinks,
    // TODO(fra): should parse to i64
    id: String,
    paging_token: String,
    seller: String,
    selling: Asset,
    buying: Asset,
    amount: String,
    #[serde(rename = "price_r")]
    price_ratio: Price,
    price: String,
    last_modified_ledger: i32,
    last_modified_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OfferLinks {
    #[serde(rename = "self")]
    self_: Link,
    offer_maker: Link,
}
