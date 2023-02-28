#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Item {
    pub volume: u64,
    pub price: u64,
    pub name: String,
    pub id: u64,
    pub alltime: u64,
}
