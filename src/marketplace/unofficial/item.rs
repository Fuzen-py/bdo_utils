use chrono::{offset::Utc, DateTime, Duration};
use serde::{Deserialize, Serialize};

/// Create an expiration time for 2h from now
pub(crate) fn expiration() -> DateTime<Utc> {
    Utc::now() + Duration::hours(2)
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Item {
    #[serde(rename = "chooseKey")]
    pub enhancement: u8,
    pub count: u64,
    //pub grade: Grade,
    #[serde(rename = "keyType")]
    pub key_type: u64,
    #[serde(rename = "mainCategory")]
    pub main_category: u64,
    #[serde(rename = "mainKey")]
    pub id: u64,
    pub name: String,
    #[serde(rename = "pricePerOne")]
    pub price: u64,
    #[serde(rename = "subCategory")]
    pub sub_category: u64,
    #[serde(rename = "subKey")]
    pub sub_key: u64,
    #[serde(rename = "totalTradeCount")]
    pub trades: u64,
    #[serde(default = "expiration")]
    pub updated: DateTime<Utc>,
    #[serde(default)]
    pub vendor_price: Option<u64>,
}
