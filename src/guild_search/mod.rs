

use crate::models::Region;
use crate::player_search::{PlayerProfile, PlayerResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ByName {
    region: Region,
    page: u16,
    search_text: String,
}

pub struct GuildQuery {
    pub name: String,
    pub token: Option<GuildToken>,
}

pub enum PlayerCache {
    PlayerQuery(PlayerResult),
    PlayerProfile(PlayerProfile),
}

pub struct Guild {
    pub region: Region,
    pub name: String,
    pub members: Vec<PlayerCache>,
}

pub struct GuildToken(pub(crate) String);

#[derive(Debug, Copy, Clone)]
pub struct GuildSearch(Region);

impl GuildSearch {
    const fn url(self) -> &'static str {
        use Region::*;
        match self.0 {
            NA | EU => "https://www.naeu.playblackdesert.com/en-US/Adventure/Guild",
        }
    }
    pub async fn by_name(self, name: String) {
        let url = self.url();
    }
}
