use crate::error::Result;
use crate::request::Request;
use crate::resources;
use stellar_base::crypto::PublicKey;
use url::Url;

#[derive(Debug, Clone)]
pub struct SingleAccountRequest {
    account_id: String,
}

pub fn single(public_key: &PublicKey) -> SingleAccountRequest {
    let account_id = public_key.account_id();
    SingleAccountRequest { account_id }
}

impl Request for SingleAccountRequest {
    type Response = resources::Account;

    fn uri(&self, host: &Url) -> Result<Url> {
        let path = format!("/accounts/{}", self.account_id);
        Ok(host.join(&path)?)
    }
}

#[cfg(test)]
mod tests {
    use super::single;
    use crate::request::Request;
    use stellar_base::crypto::PublicKey;
    use url::Url;

    #[test]
    fn test_single_request_uri() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org".parse().unwrap();
        let req = single(&pk);
        let uri = req.uri(&host).unwrap();
        assert_eq!(
            "https://horizon.stellar.org/accounts/GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA",
            uri.to_string()
        );
    }
}
