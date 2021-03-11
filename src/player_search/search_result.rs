use crate::{guild_search::Guild, models::Region};
pub struct AdventurerSearchResult {
    players: Vec<()>,
}

pub struct Profile;

pub struct PlayerResult {
    pub region: Region,
    pub family_name: String,
    pub profile_token: super::PlayerToken,
    pub level: Option<u16>,
    pub guild: Option<Guild>,
}

impl PlayerResult {
    pub async fn profile(&self) -> anyhow::Result<Option<Profile>> {
        super::PlayerSearch(self.region)
            .get_profile(self.profile_token.0.clone())
            .await
    }
    pub async fn guild(&self) -> anyhow::Result<Option<Guild>> {
        if let Some(ref _guild) = self.guild {
            todo!()
        } else {
            Ok(None)
        }
    }
}
