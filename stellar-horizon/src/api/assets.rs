use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::crypto::PublicKey;
use url::Url;

#[derive(Debug, Clone)]
pub struct AllAssetsRequest {
    asset_code: Option<String>,
    asset_issuer: Option<String>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

pub fn all() -> AllAssetsRequest {
    AllAssetsRequest {
        asset_code: None,
        asset_issuer: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

impl AllAssetsRequest {
    pub fn with_asset_code(mut self, code: &str) -> AllAssetsRequest {
        self.asset_code = Some(code.to_string());
        self
    }

    pub fn with_asset_issuer(mut self, issuer: &PublicKey) -> Result<AllAssetsRequest> {
        self.asset_issuer = Some(issuer.account_id()?);
        Ok(self)
    }
}

impl Request for AllAssetsRequest {
    type Response = Page<resources::AssetStat>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/assets")?;
        {
            let mut query = url.query_pairs_mut();
            if let Some(asset_code) = &self.asset_code {
                query.append_pair("asset_code", &asset_code);
            }
            if let Some(asset_issuer) = &self.asset_issuer {
                query.append_pair("asset_issuer", &asset_issuer);
            }
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllAssetsRequest);
