use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ItemCategory {
    Unknown,
}
