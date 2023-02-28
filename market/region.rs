#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum Region {
    NA,
    EU,
}

impl Region {
    pub(crate) fn url(self) -> &'static str {
        match self {
            Self::NA => "https://bdutils.com/api/na/items/",
            Self::EU => "https://bdutils.com/api/eu/items/",
        }
    }
}

impl Default for Region {
    fn default() -> Self {
        Self::NA
    }
}
