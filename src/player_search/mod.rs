mod player;
pub use player::Player;
mod search_result;
use serde::{Serialize, Deserialize};
use ::url::Url;
use crate::models::Region;
use search_result::Profile;

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
    Family = 2
}

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    region: Region,
    #[serde(rename = "searchType")]
    search_type: SearchType,
    #[serde(rename = "searchKeyword")]
    query: String
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
        Ok(())
    }

    /// Search for a player by character name
    pub async fn search_character(self, name: String) {

    }

    /// Search for a player by profile
    pub async fn get_profile(self, token: String) -> anyhow::Result<Profile> {
        todo!()
    }
}