use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ItemCategory;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MarketItem {
    pub id: u64,
    pub name: String,
    pub category: ItemCategory,
    pub enhancement: u16,
    pub grade: crate::models::grade::Grade,
    pub price: u64,
    pub trades: u64,
    pub stock: u64,
    pub market_weight: Option<f64>,
    pub weight: Option<f64>,
    pub updated: DateTime<Utc>,
}

impl MarketItem {
    /// If an item is ready to be updated
    /// should at least an hour before
    /// a cache'd entry is expired.
    pub fn expired(&self) -> bool {
        self.updated <= Utc::now()
    }
}
