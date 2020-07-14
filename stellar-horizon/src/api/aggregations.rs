use crate::api::Page;
use crate::error::{Error, Result};
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use chrono::{DateTime, Duration, Utc};
use std::convert::TryInto;
use stellar_base::amount::{Amount, Stroops};
use stellar_base::asset::{Asset, CreditAsset};
use stellar_base::crypto::PublicKey;
use stellar_base::error::Error as StellarBaseError;
use url::Url;

#[derive(Debug, Copy, Clone)]
pub enum Resolution {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    OneDay,
    OneWeek,
    Custom(Duration),
}

pub fn order_book(selling: Asset, buying: Asset) -> OrderBookRequest {
    OrderBookRequest {
        buying,
        selling,
        limit: None,
    }
}

pub fn paths_strict_receive<S: TryInto<Stroops>>(
    source_assets: Vec<CreditAsset>,
    destination_asset: Asset,
    destination_amount: S,
) -> Result<PathsStrictReceiveRequest> {
    let destination_amount = destination_amount
        .try_into()
        .map_err(|_| Error::StellarBaseError(StellarBaseError::InvalidStroopsAmount))?;
    Ok(PathsStrictReceiveRequest {
        source_account: None,
        source_assets,
        destination_asset,
        destination_amount,
    })
}

pub fn paths_strict_send<S: TryInto<Stroops>>(
    source_asset: Asset,
    destination_assets: Vec<CreditAsset>,
    source_amount: S,
) -> Result<PathsStrictSendRequest> {
    let source_amount = source_amount
        .try_into()
        .map_err(|_| Error::StellarBaseError(StellarBaseError::InvalidStroopsAmount))?;
    Ok(PathsStrictSendRequest {
        source_asset,
        source_amount,
        destination_assets,
        destination_account: None,
    })
}

pub fn all_trades(
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    resolution: Resolution,
    base_asset: Asset,
    counter_asset: Asset,
) -> AllTradesRequest {
    AllTradesRequest {
        start_time,
        end_time,
        resolution,
        base_asset,
        counter_asset,
        offset: None,
        order: None,
        limit: None,
    }
}

pub fn fee_stats() -> FeeStatsRequest {
    FeeStatsRequest {}
}

#[derive(Debug, Clone)]
pub struct OrderBookRequest {
    limit: Option<u64>,
    selling: Asset,
    buying: Asset,
}

#[derive(Debug, Clone)]
pub struct PathsStrictReceiveRequest {
    source_account: Option<String>,
    source_assets: Vec<CreditAsset>,
    destination_asset: Asset,
    destination_amount: Stroops,
}

#[derive(Debug, Clone)]
pub struct PathsStrictSendRequest {
    destination_account: Option<String>,
    source_asset: Asset,
    destination_assets: Vec<CreditAsset>,
    source_amount: Stroops,
}

#[derive(Debug, Clone)]
pub struct AllTradesRequest {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    resolution: Resolution,
    base_asset: Asset,
    counter_asset: Asset,
    order: Option<Order>,
    limit: Option<i64>,
    offset: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct FeeStatsRequest {}

impl OrderBookRequest {
    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl PathsStrictReceiveRequest {
    pub fn with_source_account(mut self, source_account: String) -> Self {
        self.source_account = Some(source_account);
        self
    }
}

impl PathsStrictSendRequest {
    pub fn with_destination_account(mut self, destination_account: String) -> Self {
        self.destination_account = Some(destination_account);
        self
    }
}

impl Request for OrderBookRequest {
    type Response = resources::OrderBookSummary;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/order_book")?;
        url = url.append_asset_params(&self.buying, Some("buying"))?;
        url = url.append_asset_params(&self.selling, Some("selling"))?;
        if let Some(limit) = &self.limit {
            url = url.append_query_param("limit", &limit.to_string());
        }
        Ok(url)
    }
}

impl Request for PathsStrictReceiveRequest {
    type Response = resources::Path;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/paths/strict-receive")?;
        if let Some(source_account) = &self.source_account {
            url = url.append_query_param("source_account", &source_account);
        }
        if !self.source_assets.is_empty() {
            let source_assets = serialize_assets_to_query_value(&self.source_assets)?;
            url = url.append_query_param("source_assets", &source_assets);
        }
        url = url.append_asset_params(&self.destination_asset, Some("destination"))?;
        let amount = Amount::from_stroops(&self.destination_amount)?;
        url = url.append_query_param("destination_amount", &amount.to_string());
        Ok(url)
    }
}

impl Request for PathsStrictSendRequest {
    type Response = resources::Path;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/paths/strict-send")?;
        if let Some(destination_account) = &self.destination_account {
            url = url.append_query_param("destination_account", &destination_account);
        }
        if !self.destination_assets.is_empty() {
            let destination_assets = serialize_assets_to_query_value(&self.destination_assets)?;
            url = url.append_query_param("destination_assets", &destination_assets);
        }
        url = url.append_asset_params(&self.source_asset, Some("source"))?;
        let amount = Amount::from_stroops(&self.source_amount)?;
        url = url.append_query_param("source_amount", &amount.to_string());
        Ok(url)
    }
}

impl StreamRequest for OrderBookRequest {
    type Resource = resources::OrderBookSummary;
}

impl Request for AllTradesRequest {
    type Response = Page<resources::Trade>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/trade_aggregations")?;
        let start_time = self.start_time.timestamp_millis();
        url = url.append_query_param("start_time", &start_time.to_string());
        let end_time = self.end_time.timestamp_millis();
        url = url.append_query_param("end_time", &end_time.to_string());
        let resolution = resolution_to_milliseconds(&self.resolution);
        url = url.append_query_param("resolution", &resolution.to_string());
        if let Some(offset) = &self.offset {
            url = url.append_query_param("offset", &offset.num_milliseconds().to_string());
        }
        url = url.append_asset_params(&self.base_asset, Some("base"))?;
        url = url.append_asset_params(&self.counter_asset, Some("counter"))?;
        if let Some(order) = &self.order {
            url = url.append_query_param("order", &order.to_query_value());
        }
        if let Some(limit) = &self.limit {
            url = url.append_query_param("limit", &limit.to_string());
        }
        Ok(url)
    }
}

impl Request for FeeStatsRequest {
    type Response = resources::FeeStats;

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.join("/fee_stats")?)
    }
}

fn serialize_assets_to_query_value(assets: &Vec<CreditAsset>) -> Result<String> {
    let assets: Result<Vec<_>> = assets.iter().map(credit_asset_to_string).collect();
    Ok(assets?.join(","))
}

fn credit_asset_to_string(asset: &CreditAsset) -> Result<String> {
    let code = asset.code();
    let issuer = asset.issuer().account_id()?;
    Ok(format!("{}:{}", code, issuer))
}

fn resolution_to_milliseconds(resolution: &Resolution) -> u64 {
    match resolution {
        Resolution::OneMinute => 60000,
        Resolution::FiveMinutes => 300000,
        Resolution::FifteenMinutes => 900000,
        Resolution::OneHour => 3600000,
        Resolution::OneDay => 86400000,
        Resolution::OneWeek => 604800000,
        Resolution::Custom(d) => d.num_milliseconds() as u64,
    }
}
