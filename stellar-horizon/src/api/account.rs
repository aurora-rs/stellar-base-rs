use crate::client::{Request, StreamRequest};
use crate::error::Result;
use crate::resources;
use stellar_base::crypto::PublicKey;

pub struct SingleAccountRequest {
    account_id: String,
}

pub fn single(public_key: &PublicKey) -> Result<SingleAccountRequest> {
    let account_id = public_key.account_id()?;
    Ok(SingleAccountRequest { account_id })
}

impl Request for SingleAccountRequest {
    type Response = resources::Account;

    fn uri(&self, host: &str) -> Result<String> {
        Ok(format!("{}/accounts/{}", host, self.account_id))
    }
}
