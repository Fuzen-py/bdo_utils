use chrono::{Date, Utc};

use crate::{
    guild_search::{Guild, GuildQuery, GuildToken},
    models::{lifeskill_level::LifeSkillLevel, Region},
};
pub struct AdventurerSearchResult {
    players: Vec<()>,
}

pub(crate) enum GuildCache {
    Unprocessed(GuildQuery),
    Processed(Guild),
}

pub struct CharacterLifeskills {
    gathering: LifeSkillLevel,
    fishing: LifeSkillLevel,
    hunting: LifeSkillLevel,
    cooking: LifeSkillLevel,
    alchemy: LifeSkillLevel,
    processing: LifeSkillLevel,
    taming: LifeSkillLevel,
    trading: LifeSkillLevel,
    farming: LifeSkillLevel,
    sailing: LifeSkillLevel,
    bartering: LifeSkillLevel,
}

pub struct Character {
    name: String,
    class: String,
    contribution: Option<u16>,
    level: Option<u16>,
    lifeskills: Option<CharacterLifeskills>,
    is_main: bool,
}

pub struct Profile {
    pub(crate) guild: Option<GuildCache>,
    pub crated: Date<Utc>,
    pub characters: Vec<Character>,
}

pub struct PlayerResult {
    /// Player's Region
    pub region: Region,
    /// Family Name of the account
    pub family: String,
    /// Profile Token (This is static afaik)
    pub token: super::PlayerToken,
    /// Selected Main's level
    pub level: Option<u16>,
    /// Selected Main's class
    pub class: String,
    /// Players Guild, may be private
    pub guild: Option<GuildQuery>,
}

impl PlayerResult {
    pub async fn profile(&self) -> anyhow::Result<Option<Profile>> {
        super::PlayerSearch(self.region)
            .get_profile(self.token.0.clone())
            .await
    }
    pub async fn guild(&self) -> anyhow::Result<Option<GuildQuery>> {
        if let Some(ref _guild) = self.guild {
            todo!()
        } else {
            Ok(None)
        }
    }
}
