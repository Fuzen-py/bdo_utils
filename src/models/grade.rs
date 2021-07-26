use serde_repr::{Deserialize_repr, Serialize_repr};
/// Represents an item grade
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
#[serde(default = "Grade::Unknown")]
pub enum Grade {
    White = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Orange = 4,
    Unknown,
}
