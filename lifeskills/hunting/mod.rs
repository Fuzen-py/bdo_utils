pub mod items;
use self::items::{HuntersClothes, HuntingOutfit};
use super::Hedgehog;
use crate::lifeskills::LifeSkill;
use crate::lifeskills::MasteryLevel;

const MEAT_DROP: f32 = 6.5;
const BLOOD_DROP: f32 = 2.0;
const HIDE_DROP: f32 = 1.6;

pub(crate) fn meat_from_hedgehog(hedgehog: Hedgehog, kills: u32) -> u32 {
    (MEAT_DROP * hedgehog.proc_chance() * kills as f32).floor() as u32
}

pub(crate) fn blood_from_hedgehog(hedgehog: Hedgehog, kills: u32) -> u32 {
    (BLOOD_DROP * hedgehog.proc_chance() * kills as f32).floor() as u32
}

pub(crate) fn hide_from_hedgehog(hedgehog: Hedgehog, kills: u32) -> u32 {
    (HIDE_DROP * hedgehog.proc_chance() * kills as f32).floor() as u32
}

fn increase_in_items(mastery: u32) -> f32 {
    match mastery {
        n if n >= 2000 => 300.0,
        1950..=1999 => 289.7,
        1900..=1949 => 279.4,
        1850..=1899 => 269.3,
        1800..=1849 => 259.3,
        1750..=1799 => 249.4,
        1700..=1749 => 239.6,
        1650..=1699 => 230.0,
        1600..=1649 => 220.5,
        1550..=1599 => 211.1,
        1500..=1549 => 201.9,
        1450..=1499 => 192.8,
        1400..=1449 => 183.9,
        1350..=1399 => 175.2,
        1300..=1340 => 166.6,
        1250..=1299 => 158.1,
        1200..=1249 => 149.8,
        1150..=1199 => 141.7,
        1100..=1149 => 133.8,
        1050..=1099 => 126.1,
        1000..=1049 => 118.5,
        950..=999 => 111.2,
        900..=949 => 104.0,
        850..=899 => 97.0,
        800..=849 => 90.2,
        750..=799 => 83.7,
        700..=749 => 77.3,
        650..=699 => 71.2,
        600..=649 => 65.3,
        550..=599 => 59.6,
        500..=549 => 54.1,
        450..=499 => 48.9,
        400..=449 => 43.9,
        350..=399 => 39.1,
        300..=349 => 34.6,
        250..=299 => 30.3,
        200..=249 => 26.4,
        150..=199 => 22.6,
        100..=149 => 19.1,
        50..=99 => 15.0,
        0..=49 => 0.0,
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Hunter {
    pub mastery_level: MasteryLevel,
    pub clothes: HuntersClothes,
    pub hunting_outfit: HuntingOutfit,
}

impl Hunter {
    pub fn mastery(self, life: LifeSkill) -> u32 {
        life.mastery() + self.mastery_level.mastery() + self.clothes.mastery()
    }
    pub fn xp_boost(self, life: LifeSkill) -> f32 {
        let outfit_bonus = self.hunting_outfit.xp_boost();
        self.clothes.xp_boost() + outfit_bonus + life.xp_boost()
    }
    pub fn meat_hour(self, life: LifeSkill, kills: u32) -> u32 {
        let mastery = self.mastery(life);
        // Increase in items from mastery
        let incr = (increase_in_items(mastery) / 100.0) + 1.0;
        // 6.5 meat per wolf on avg @ base
        let meat = (kills as f32 * MEAT_DROP * incr).floor() as u32;
        // Add in meat from hedgehog (not affected by mastery)
        meat + meat_from_hedgehog(life.hedgehog, kills)
    }

    pub fn blood_hour(self, life: LifeSkill, kills: u32) -> u32 {
        let mastery = self.mastery(life);
        let incr = (increase_in_items(mastery) / 100.0) + 1.0;
        let blood = (kills as f32 * BLOOD_DROP * incr).floor() as u32;
        blood + blood_from_hedgehog(life.hedgehog, kills)
    }

    pub fn hide_hour(self, life: LifeSkill, kills: u32) -> u32 {
        let mastery = self.mastery(life);
        let incr = (increase_in_items(mastery) / 100.0) + 1.0;
        let hide = (kills as f32 * HIDE_DROP * incr).floor() as u32;
        hide + hide_from_hedgehog(life.hedgehog, kills)
    }

    pub fn average_xp(self, life: LifeSkill) -> u64 {
        let incr = increase_in_items(self.mastery(life));
        let xp_boost = (self.xp_boost(life) + incr + life.hedgehog.xp_boost()) / 100.0;
        ((xp_boost + 1.0) * 3200.0).floor() as u64
    }
    pub fn xp_hour(self, life: LifeSkill, kills: u32) -> u64 {
        self.average_xp(life) * kills as u64
    }
    pub fn kills_to_next(self, life: LifeSkill) -> u64 {
        (self.mastery_level.xp_to_next() as f64 / self.average_xp(life) as f64).ceil() as u64
    }
    pub fn kills_to(self, life: LifeSkill, goal: MasteryLevel) -> u64 {
        // TODO: Account for mastery changes between current & goal
        (self.mastery_level.xp_to(goal) as f64 / self.average_xp(life) as f64).ceil() as u64
    }
}
