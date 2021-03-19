use chrono::NaiveDate;

use crate::{
    guild_search::{Guild, GuildQuery},
    models::{lifeskill_level::LifeSkillLevel, Region},
    PlayerSearch, PlayerToken,
};

pub(crate) enum GuildCache {
    Unprocessed(GuildQuery),
    Processed(Guild),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CharacterLifeskills {
    pub gathering: LifeSkillLevel,
    pub fishing: LifeSkillLevel,
    pub hunting: LifeSkillLevel,
    pub cooking: LifeSkillLevel,
    pub alchemy: LifeSkillLevel,
    pub processing: LifeSkillLevel,
    pub training: LifeSkillLevel,
    pub trading: LifeSkillLevel,
    pub farming: LifeSkillLevel,
    pub sailing: LifeSkillLevel,
    pub bartering: LifeSkillLevel,
}

pub struct Character {
    pub name: String,
    pub class: String,
    pub contribution: Option<u16>,
    pub level: Option<u16>,
    pub lifeskills: Option<CharacterLifeskills>,
    pub is_main: bool,
}

pub struct Profile {
    pub name: String,
    pub(crate) guild: Option<GuildCache>,
    pub created: NaiveDate,
    pub characters: Vec<Character>,
    pub token: PlayerToken,
    pub region: Region,
}

impl Profile {
    /// Get a mutable reference to the profile's guild.
    pub fn guild_mut(&mut self) -> Option<&mut Guild> {
        if let Some(ref mut gcache) = self.guild {
            match gcache {
                GuildCache::Processed(g) => Some(g),
                GuildCache::Unprocessed(ref _query) => unimplemented!(),
            }
        } else {
            None
        }
    }
    pub async fn update(&mut self) -> anyhow::Result<()> {
        let token = self.token.0.clone();
        if let Some(profile) = PlayerSearch::init(self.region).get_profile(token).await? {
            *self = profile;
        }
        Ok(())
    }
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
    /// Main character's name
    pub main_name: String,
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
