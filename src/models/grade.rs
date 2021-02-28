// Not sure if this type will ever be actually used

/// Represents an item grade
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Grade {
    White = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Orange = 4,
}