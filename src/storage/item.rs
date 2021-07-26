use std::u128;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Item(pub u128);

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemWeight {
    pub item: Item,
    pub weight: f32,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemPrice {
    pub item: Item,
    pub buy: Option<u128>,
    pub sell: Option<u128>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemName {
    pub language: String,
    pub item: Item,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub item: Item,
    pub name: ItemName,
    pub price: ItemPrice,
    pub weight: ItemWeight,
}

impl AsRef<Item> for ItemInfo {
    fn as_ref(&self) -> &Item {
        &self.item
    }
}
impl AsRef<ItemName> for ItemInfo {
    fn as_ref(&self) -> &ItemName {
        &self.name
    }
}
impl AsRef<ItemPrice> for ItemInfo {
    fn as_ref(&self) -> &ItemPrice {
        &self.price
    }
}
impl AsRef<ItemWeight> for ItemInfo {
    fn as_ref(&self) -> &ItemWeight {
        &self.weight
    }
}
