use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    id: String,
    account_id: String,
    sequence: String,
}
