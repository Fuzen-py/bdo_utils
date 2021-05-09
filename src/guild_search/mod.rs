use crate::models::Region;
use crate::player_search::{PlayerProfile, PlayerResult};
use serde::{Deserialize, Serialize};
mod html_parse;

pub struct GuildPreview {}

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
pub struct GuildSearch(pub Region);

impl GuildSearch {
    const fn url(self) -> &'static str {
        use Region::*;
        match self.0 {
            NA | EU => "https://www.naeu.playblackdesert.com/en-US/Adventure/Guild",
        }
    }
    pub async fn by_name(self, name: &str) -> anyhow::Result<Option<Guild>> {
        let client = reqwest::Client::new();
        let url = ::url::Url::parse(self.url()).unwrap();
        let query = ByName {
            region: self.0,
            search_text: name.to_owned(),
            page: 1,
        };
        let resp = client.get(url).query(&query).send().await?;
        let _body = resp.text().await?;
        unimplemented!()
    }
    pub async fn get(self, _token: &str) -> anyhow::Result<Option<Guild>> {
        unimplemented!()
    }
}

impl Guild {
    // async fn players(&mut self) -> Iterator<Item=&mut PlayerProfile> {
    //     self.members.iter_mut().filter_map(|c| {
    //         match c {
    //             PlayerCache::PlayerProfile(ref mut p) => p,
    //             PlayerCache::PlayerQuery(r) => {
    //                 PlayerResult()
    //             }

    //         }
    //     })
    // }
}
