use crate::error::Result;
use crate::request::{Request, StreamRequest};
use crate::resources;
use stellar_base::crypto::PublicKey;
use url::Url;

pub struct SingleAccountRequest {
    account_id: String,
}

pub fn single(public_key: &PublicKey) -> Result<SingleAccountRequest> {
    let account_id = public_key.account_id()?;
    Ok(SingleAccountRequest { account_id })
}

impl Request for SingleAccountRequest {
    type Response = resources::Account;

    fn uri(&self, host: &Url) -> Result<Url> {
        let path = format!("/accounts/{}", self.account_id);
        Ok(host.join(&path)?)
    }
}
