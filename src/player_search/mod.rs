mod player;
pub use player::Player;
mod search_result;
use crate::models::Region;
use search_result::Profile;
use serde::{Deserialize, Serialize};
use url::Url;
mod html_parse;

#[derive(Debug, Clone, Copy)]
pub struct PlayerSearch(Region);
impl From<Region> for PlayerSearch {
    fn from(region: Region) -> Self {
        Self(region)
    }
}

#[derive(Serialize, Deserialize)]
#[repr(u8)]
enum SearchType {
    Character = 1,
    Family = 2,
}

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    region: Region,
    #[serde(rename = "searchType")]
    search_type: SearchType,
    #[serde(rename = "searchKeyword")]
    query: String,
}

#[derive(Serialize, Deserialize)]
struct ProfileTarget {
    #[serde(rename = "profileTarget")]
    profile_target: String,
}

pub(crate) struct PageBody(String);

pub struct PlayerToken(String);
impl From<String> for PlayerToken {
    fn from(token: String) -> Self {
        PlayerToken(token)
    }
}

impl PlayerSearch {
    pub fn init(region: Region) -> Self {
        Self(region)
    }

    /// Search for a player by family name
    pub async fn search_family(self, name: String) -> anyhow::Result<()> {
        // https://www.naeu.playblackdesert.com/en-US/Adventure?region={region}&searchType=2&searchKeyword={name}
        let client = reqwest::Client::new();
        let url = Url::parse(self.0.official_site())?.join("Adventure")?;
        let query = SearchQuery {
            region: self.0,
            search_type: SearchType::Family,
            query: name,
        };
        let resp = client.get(url).query(&query).send().await?;
        let body = resp.text().await?;
        let _players = html_parse::prase_search_page(&body);
        Ok(())
    }

    /// Search for a player by character name
    pub async fn search_character(self, _name: String) {}

    /// Search for a player by profile
    pub async fn get_profile(self, token: String) -> anyhow::Result<Option<Profile>> {
        let profile_page = reqwest::Client::new()
            .get(get_url(self.0))
            .query(&ProfileTarget {
                profile_target: token,
            })
            .send()
            .await?
            .text()
            .await?;
        if profile_page.contains(r#"alert("This profile does not exist.")"#) {
            return Ok(None);
        }
        // TODO: Parse HTML Result
        todo!()
    }
}

const fn get_url(region: Region) -> &'static str {
    match region {
        Region::NA => "https://www.naeu.playblackdesert.com/en-US/Adventure",
        Region::EU => "https://www.naeu.playblackdesert.com/en-US/Adventure",
    }
}
