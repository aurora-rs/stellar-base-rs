use crate::resources::account::AccountFlags;
use crate::resources::{Asset, Link};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetStat {
    #[serde(rename = "_links")]
    links: AssetStatLinks,
    #[serde(flatten)]
    asset: Asset,
    paging_token: String,
    amount: String,
    num_accounts: i32,
    flags: AccountFlags,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetStatLinks {
    toml: Link,
}
