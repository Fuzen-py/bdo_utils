use crate::models::MarketItem;
use async_trait::async_trait;

/// Trait to handle data management
#[async_trait]
pub trait DataManager {
    /// Initiate the data manager
    async fn init() -> Self;
    /// Cache an item fetched from the central marketplace
    async fn store_market_item(item: MarketItem);
    /// Retrieve a cache'd item
    async fn fetch_market_item(id: u64) -> Option<MarketItem>;
}
