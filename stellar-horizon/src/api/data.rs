use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::crypto::PublicKey;
use url::Url;

pub fn for_account<S: Into<String>>(account: &PublicKey, key: S) -> DataForAccountRequest {
    DataForAccountRequest {
        account_id: account.account_id(),
        key: key.into(),
    }
}

#[derive(Debug, Clone)]
pub struct DataForAccountRequest {
    account_id: String,
    key: String,
}

impl Request for DataForAccountRequest {
    type Response = resources::AccountData;

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.join(&format!("/accounts/{}/data/{}", self.account_id, self.key))?)
    }
}
