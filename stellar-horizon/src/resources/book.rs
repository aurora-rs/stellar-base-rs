use crate::resources::account::AccountFlags;
use crate::resources::{Asset, Link, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderBookSummary {
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>,
    base: Asset,
    counter: Asset,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceLevel {
    #[serde(rename = "price_r")]
    price_ratio: Price,
    price: String,
    amount: String,
}
