use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Region {
    NA,
    EU,
}

impl Default for Region {
    fn default() -> Self {
        Region::NA
    }
}

impl Region {
    pub const fn official_site(self) -> &'static str {
        match self {
            Self::NA | Self::EU => "https://www.naeu.playblackdesert.com/en-US/",
        }
    }
}
