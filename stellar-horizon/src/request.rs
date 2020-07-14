use crate::error::Result;
use serde::de::DeserializeOwned;
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

pub(crate) trait UrlPageRequestExt {
    fn append_pagination_params<R: PageRequest>(self, req: &R) -> Self;
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
