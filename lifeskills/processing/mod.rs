use crate::lifeskills::{LifeSkill, MasteryLevel};

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Processing {
    mastery_lvl: MasteryLevel,
}

impl Processing {
    pub fn materials_count(self, life: LifeSkill) -> u8 {
        match self.mastery(life) {
            2..=19 => 10,
            20..=39 => 11,
            40..=59 => 12,
            60..=79 => 13,
            80..=99 => 14,
            100..=119 => 15,
            120..=139 => 16,
            140..=159 => 17,
            160..=179 => 18,
            180..=199 => 19,
            200..=219 => 20,
            220..=239 => 21,
            240..=259 => 22,
            260..=279 => 23,
            280..=299 => 24,
            300..=319 => 26,
            _ => unimplemented!(),
        }
    }
    pub fn mastery(self, life: LifeSkill) -> u32 {
        life.mastery() + self.mastery_lvl.mastery()
    }
}
