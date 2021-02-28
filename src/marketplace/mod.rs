use serde::{Deserialize, Serialize};
//mod official;
mod unofficial;

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
