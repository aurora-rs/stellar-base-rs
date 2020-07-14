use crate::error::Result;
use serde::de::DeserializeOwned;
use stellar_base::asset::{Asset, CreditAssetType};
use url::Url;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Order {
    Ascending,
    Descending,
}

pub trait Request: Send + Sync {
    type Response: DeserializeOwned;

    fn is_post(&self) -> bool {
        false
    }

    fn uri(&self, host: &Url) -> Result<Url>;
}

pub trait PageRequest: Request {
    fn with_cursor(self, cursor: &str) -> Self;
    fn cursor(&self) -> &Option<String>;

    fn with_limit(self, limit: u64) -> Self;
    fn limit(&self) -> &Option<u64>;

    fn with_order(self, direction: &Order) -> Self;
    fn order(&self) -> &Option<Order>;
}

pub trait StreamRequest: Request + Unpin {
    type Resource: DeserializeOwned + Send + Sync;
}

pub(crate) trait UrlPageRequestExt: Sized {
    fn append_pagination_params<R: PageRequest>(self, req: &R) -> Self;
    fn append_asset_params(self, asset: &Asset, prefix: Option<&str>) -> Result<Self>;
    fn append_query_param(self, key: &str, value: &str) -> Self;
}

impl UrlPageRequestExt for Url {
    fn append_pagination_params<R: PageRequest>(mut self, req: &R) -> Self {
        {
            let mut query = self.query_pairs_mut();
            if let Some(cursor) = req.cursor() {
                query.append_pair("cursor", cursor);
            }

            if let Some(limit) = req.limit() {
                query.append_pair("limit", &limit.to_string());
            }

            if let Some(order) = req.order() {
                let order = match order {
                    Order::Ascending => "asc",
                    Order::Descending => "desc",
                };
                query.append_pair("order", order);
            }
        }
        self
    }

    fn append_asset_params(mut self, asset: &Asset, prefix: Option<&str>) -> Result<Self> {
        {
            let mut query = self.query_pairs_mut();

            let asset_type = match asset {
                Asset::Native => "native",
                Asset::Credit(credit) => match credit.asset_type() {
                    CreditAssetType::CreditAlphaNum4(_) => "credit_alphanum4",
                    CreditAssetType::CreditAlphaNum12(_) => "credit_alphanum12",
                },
            };

            if let Some(prefix) = prefix {
                query.append_pair(&format!("{}_asset_type", prefix), asset_type);
            } else {
                query.append_pair("asset_type", asset_type);
            }

            if let Asset::Credit(credit) = asset {
                let asset_code = credit.code();
                let asset_issuer = credit.issuer().account_id()?;
                if let Some(prefix) = prefix {
                    query.append_pair(&format!("{}_asset_code", prefix), asset_code);
                    query.append_pair(&format!("{}_asset_issuer", prefix), &asset_issuer);
                } else {
                    query.append_pair("asset_code", asset_code);
                    query.append_pair("asset_ssuer", &asset_issuer);
                }
            }
        }
        Ok(self)
    }

    fn append_query_param(mut self, key: &str, value: &str) -> Self {
        {
            let mut query = self.query_pairs_mut();
            query.append_pair(key, value);
        }
        self
    }
}

impl Order {
    pub fn to_query_value(&self) -> String {
        match self {
            Order::Ascending => "asc".to_string(),
            Order::Descending => "desc".to_string(),
        }
    }
}

macro_rules! impl_page_request {
    ($name:path) => {
        impl PageRequest for $name {
            fn with_cursor(mut self, cursor: &str) -> Self {
                self.cursor = Some(cursor.to_string());
                self
            }

            fn cursor(&self) -> &Option<String> {
                &self.cursor
            }

            fn with_limit(mut self, limit: u64) -> Self {
                self.limit = Some(limit);
                self
            }

            fn limit(&self) -> &Option<u64> {
                &self.limit
            }

            fn with_order(mut self, order: &Order) -> Self {
                self.order = Some(order.clone());
                self
            }

            fn order(&self) -> &Option<Order> {
                &self.order
            }
        }
    };
}
