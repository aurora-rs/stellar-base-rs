// All resources have the same type (when possible) of the
// horizon protocol definition at
// https://github.com/stellar/go/blob/master/protocols/horizon/
//
// When updating, use that as your source of truth.
pub mod account;
pub mod asset;
pub mod ledger;
pub mod offer;
pub mod root;
pub mod trade;
pub mod transaction;

pub use account::*;
pub use asset::*;
pub use ledger::*;
pub use offer::*;
pub use root::*;
pub use trade::*;
pub use transaction::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Price {
    #[serde(rename = "n")]
    numerator: i32,
    #[serde(rename = "d")]
    denominator: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Asset {
    asset_type: String,
    asset_code: Option<String>,
    asset_isuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Path {
    #[serde(flatten, with = "SourceAsset")]
    source_asset: Asset,
    souce_amount: String,
    #[serde(flatten, with = "DestinationAsset")]
    destination_asset: Asset,
    destination_amount: String,
    path: Vec<Asset>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    href: String,
    #[serde(default = "default_templated_as_false")]
    templated: bool,
}

fn default_templated_as_false() -> bool {
    false
}

// https://github.com/serde-rs/serde/issues/970#issuecomment-312282671
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct SourceAsset {
    #[serde(rename = "source_asset_type")]
    asset_type: String,
    #[serde(rename = "source_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "source_asset_issuer")]
    asset_isuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct DestinationAsset {
    #[serde(rename = "destination_asset_type")]
    asset_type: String,
    #[serde(rename = "destination_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "destination_asset_issuer")]
    asset_isuer: Option<String>,
}
