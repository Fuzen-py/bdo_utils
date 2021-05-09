mod search_result;
use crate::models::Region;
use serde::{Deserialize, Serialize};
//use ::url::Url;

pub use self::search_result::{PlayerResult, Profile as PlayerProfile};
mod html_parse;

#[derive(Debug, Clone, Copy)]
pub struct PlayerSearch(pub(crate) Region);
impl From<Region> for PlayerSearch {
    fn from(region: Region) -> Self {
        Self(region)
    }
}

pub mod consts {
    use crate::{PlayerSearch, models::Region};    
    pub const NA_PLAYER_SEARCH: PlayerSearch = PlayerSearch(Region::NA);

    pub const EU_PLAYER_SEARCH: PlayerSearch = PlayerSearch(Region::EU);
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

pub struct PlayerToken(String);
impl From<String> for PlayerToken {
    fn from(token: String) -> Self {
        PlayerToken(token)
    }
}

impl<'t> From<&'t str> for PlayerToken {
    fn from(token: &'t str) -> Self {
        PlayerToken(token.to_owned())
    }
}

impl PlayerSearch {
    pub fn init(region: Region) -> Self {
        Self(region)
    }

    async fn search(self, name: String, stype: SearchType) -> anyhow::Result<Vec<PlayerResult>> {
        // https://www.naeu.playblackdesert.com/en-US/Adventure?region={region}&searchType=2&searchKeyword={name}
        let client = reqwest::Client::new();
        let url = ::url::Url::parse(self.0.official_site())?.join("Adventure")?;
        let query = SearchQuery {
            region: self.0,
            search_type: stype,
            query: name,
        };
        let resp = client.get(url).query(&query).send().await?;
        let body = resp.text().await?;
        let players = html_parse::prase_search_page(&body);
        Ok(players)
    }

    /// Search for a player by family name
    pub async fn by_family(self, name: String) -> anyhow::Result<Vec<PlayerResult>> {
        self.search(name, SearchType::Family).await
    }

    /// Search for a player by character name
    pub async fn by_character(self, name: String) -> anyhow::Result<Vec<PlayerResult>> {
        self.search(name, SearchType::Character).await
    }

    /// Search for a player by profile
    pub async fn get_profile(self, token: &str) -> anyhow::Result<Option<PlayerProfile>> {
        let profile_page = reqwest::Client::new()
            .get(get_url(self.0))
            .query(&ProfileTarget {
                profile_target: token.to_owned(),
            })
            .send()
            .await?
            .text()
            .await?;
        if profile_page.contains(r#"alert("This profile does not exist.")"#) {
            return Ok(None);
        }
        // TODO: Parse HTML Result
        Ok(html_parse::parse_profile_page(
            &profile_page,
            token.into(),
            self.0,
        ))
    }
}

const fn get_url(region: Region) -> &'static str {
    match region {
        Region::NA => "https://www.naeu.playblackdesert.com/en-US/Adventure",
        Region::EU => "https://www.naeu.playblackdesert.com/en-US/Adventure",
    }
}
