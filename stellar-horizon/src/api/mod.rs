use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use serde::ser::Serialize;

pub mod accounts;
pub mod aggregations;
pub mod assets;
pub mod data;
pub mod effects;
pub mod ledgers;
pub mod offers;
pub mod operations;
pub mod payments;
pub mod trades;
pub mod transactions;

#[derive(Debug, Clone)]
pub struct Page<T>
where
    T: DeserializeOwned + Serialize,
{
    records: Vec<T>,
}

impl<T> Page<T>
where
    T: DeserializeOwned + Serialize,
{
    pub fn records(&self) -> &Vec<T> {
        &self.records
    }
}

impl<'de, T> Deserialize<'de> for Page<T>
where
    T: DeserializeOwned + Serialize,
{
    fn deserialize<D>(d: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner: Embedded<EmbeddedRecords<T>> = Embedded::deserialize(d)?;
        Ok(Page {
            records: inner.embedded.records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Embedded<T> {
    #[serde(rename = "_embedded")]
    embedded: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddedRecords<T> {
    records: Vec<T>,
}
