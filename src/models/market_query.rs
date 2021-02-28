/// Data type used for making a query
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MarketQuery {
    ID {id: u64, enchant: u8},
    Name {name: String, enchant: u8},
}