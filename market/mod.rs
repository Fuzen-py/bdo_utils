mod item;
mod region;
pub use item::Item;
pub use region::Region;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Market {
    region: Region,
    items: Vec<Item>,
}

impl Market {
    pub async fn fetch(region: Region) -> Result<Self, reqwest::Error> {
        let mut items = reqwest::get(region.url())
            .await?
            .json::<Vec<Item>>()
            .await?;
        items.sort_by(|li, ri| {
            li.name
                .to_ascii_lowercase()
                .cmp(&ri.name.to_ascii_lowercase())
        });
        Ok(Self { region, items })
    }
    pub fn by_name(&self, name: &str) -> Option<Item> {
        self.items
            .iter()
            .find(|item| item.name.eq_ignore_ascii_case(name))
            .cloned()
    }
    pub fn by_id(&self, id: u64) -> Option<Item> {
        self.items
            .iter()
            .find(|item| item.id == id)
            .filter(|item| item.id == id)
            .cloned()
    }
    pub fn search(&self, name: &str) -> Vec<&Item> {
        let l_name = name.to_ascii_lowercase();
        self.items
            .iter()
            .filter(|i| i.name.to_ascii_lowercase().contains(&l_name))
            .collect()
    }
    pub async fn refresh(&mut self) -> Result<(), reqwest::Error> {
        self.items = reqwest::get(self.region.url())
            .await?
            .json::<Vec<Item>>()
            .await?;
        self.items.sort_by(|li, ri| li.name.cmp(&ri.name));
        Ok(())
    }
}
