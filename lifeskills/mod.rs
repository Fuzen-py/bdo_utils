pub mod hunting;
pub mod items;
pub mod mastery;
pub mod processing;
pub use hunting::Hunter;
pub use mastery::MasteryLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Hedgehog {
    T1,
    T2,
    T3,
    T4,
    None,
}

impl Hedgehog {
    pub fn proc_chance(self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::T1 => 0.2,
            Self::T2 => 0.3,
            Self::T3 => 0.4,
            Self::T4 => 0.5,
        }
    }
    pub fn xp_boost(self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::T1 => 20.0,
            Self::T2 => 30.0,
            Self::T3 => 40.0,
            Self::T4 => 50.0,
        }
    }
}

impl Default for Hedgehog {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct LifeSkill {
    pub gathering: (),
    pub fishing: (),
    pub hunting: hunting::Hunter,
    pub cooking: (),
    pub alchemy: (),
    pub processing: (),
    pub training: (),
    pub trade: (),
    pub farming: (),
    pub sailing: (),
    pub beginner: (),
    pub accessories: [items::LifeskillAccessories; 6],
    pub hedgehog: Hedgehog,
}

impl LifeSkill {
    pub fn mastery(self) -> u32 {
        self.accessories
            .iter()
            .fold(0, |m: u32, a: &items::LifeskillAccessories| m + a.mastery())
    }
    fn xp_boost(self) -> f32 {
        let mut xp = self
            .accessories
            .iter()
            .fold(0.0, |x: f32, a: &items::LifeskillAccessories| {
                x + a.xp_bonus()
            });
        // Requires 2 items per set
        let manos_ct =
            (self.accessories.iter().filter(|a| a.is_manos()).count() as f32 * 0.5).floor();
        let geranoa_ct =
            (self.accessories.iter().filter(|a| a.is_geranoa()).count() as f32 * 0.5).floor();
        let loggia_ct =
            (self.accessories.iter().filter(|a| a.is_loggia()).count() as f32 * 0.5).floor();

        xp += manos_ct * items::MANOS_SET_EFFECT;
        xp += geranoa_ct * items::GERANOA_SET_EFFECT;
        xp += loggia_ct * items::LOGGIA_SET_EFFECT;
        xp
    }
}
