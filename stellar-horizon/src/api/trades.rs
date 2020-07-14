use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::asset::Asset;
use stellar_base::crypto::PublicKey;
use url::Url;

#[derive(Debug, Clone)]
pub struct AllTradesRequest {
    offer_id: Option<String>,
    base_asset: Option<Asset>,
    counter_asset: Option<Asset>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

#[derive(Debug, Clone)]
pub struct TradesForAccountRequest {}

pub fn all() -> AllTradesRequest {
    AllTradesRequest {
        offer_id: None,
        base_asset: None,
        counter_asset: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

impl AllTradesRequest {
    pub fn with_offer_id(mut self, offer_id: &str) -> AllTradesRequest {
        self.offer_id = Some(offer_id.to_string());
        self
    }

    pub fn with_base_asset(mut self, asset: Asset) -> AllTradesRequest {
        self.base_asset = Some(asset);
        self
    }

    pub fn with_counter_asset(mut self, asset: Asset) -> AllTradesRequest {
        self.counter_asset = Some(asset);
        self
    }
}

impl Request for AllTradesRequest {
    type Response = Page<resources::Trade>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/trades")?;
        {
            let mut query = url.query_pairs_mut();
            if let Some(offer_id) = &self.offer_id {
                query.append_pair("offer_id", offer_id);
            }
        }
        if let Some(asset) = &self.base_asset {
            url = url.append_asset_params(&asset, Some("base_"))?;
        }
        if let Some(asset) = &self.counter_asset {
            url = url.append_asset_params(&asset, Some("counter_"))?;
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllTradesRequest);

impl StreamRequest for AllTradesRequest {
    type Resource = resources::Trade;
}
