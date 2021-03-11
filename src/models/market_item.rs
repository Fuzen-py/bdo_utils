use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ItemCategory;
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MarketItem {
    id: u64,
    name: String,
    category: ItemCategory,
    enhancement: u16,
    grade: u8,
    price: u64,
    trades: u64,
    stock: u64,
    updated: DateTime<Utc>,
}

impl MarketItem {
    /// If an item is ready to be updated
    /// should at least an hour before
    /// a cache'd entry is expired.
    pub fn expired(&self) -> bool {
        self.updated <= Utc::now()
    }
}
