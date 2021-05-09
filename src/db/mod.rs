use std::sync::Arc;

use crate::models::MarketItem;
use async_trait::async_trait;
use tokio::sync::RwLock;

/// Trait to handle data management
#[async_trait]
pub trait DataManager {
    /// Initiate the data manager
    async fn init() -> Self;
    /// Cache an item fetched from the central marketplace
    async fn store_market_item(&mut self, item: MarketItem) -> Option<u64>;
    /// Retrieve a cache'd item
    async fn fetch_market_item(&self, id: u64) -> Option<MarketItem>;
}

