use super::Class;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Family {
    /// Family ID
    pub id: u64,
    /// Family Name
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// Family ID
    pub family: u64,
    /// Character Name
    pub name: String,
    /// Character ID
    pub id: u64,
    // Character's class
    class: Class,
}
