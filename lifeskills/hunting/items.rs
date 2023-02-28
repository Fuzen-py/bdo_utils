use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum HuntersClothes {
    Manos { enhancement: u8 },
    Robeau { enhancement: u8 },
    Loggia { enhancement: u8 },
    None,
}

impl HuntersClothes {
    pub fn xp_boost(self) -> f32 {
        match self {
            Self::Manos { enhancement } => match enhancement {
                0..=3 => 5.0,
                4..=10 => 10.0,
                11..=15 => 15.0,
                16 | 17 => 20.0,
                18 | 19 => 25.0,
                20 => 40.0,
                _ => unreachable!(),
            },
            _ => 0.0,
        }
    }
    pub fn mastery(self) -> u32 {
        match self {
            Self::Manos { enhancement } => match enhancement {
                0 => 5,
                1 => 10,
                2 => 15,
                3 => 20,
                4 => 25,
                5 => 30,
                6 => 35,
                7 => 40,
                8 => 45,
                9 => 50,
                10 => 55,
                11 => 60,
                12 => 70,
                13 => 80,
                14 => 90,
                15 => 100,
                16 => 120,
                17 => 160,
                18 => 220,
                19 => 300,
                20 => 400,
                _ => unreachable!(),
            },
            Self::Robeau { enhancement } => match enhancement {
                0 => 4,
                1 => 8,
                2 => 12,
                3 => 16,
                4 => 20,
                5 => 24,
                6 => 28,
                7 => 32,
                8 => 36,
                9 => 40,
                10 => 44,
                11 => 50,
                12 => 58,
                13 => 66,
                14 => 74,
                15 => 80,
                16 => 95,
                17 => 125,
                18 => 180,
                19 => 250,
                20 => 330,
                _ => unreachable!(),
            },
            Self::Loggia { enhancement } => match enhancement {
                0 => 3,
                1 => 6,
                2 => 9,
                3 => 12,
                4 => 15,
                5 => 18,
                6 => 21,
                7 => 24,
                8 => 27,
                9 => 30,
                10 => 33,
                11 => 39,
                12 => 45,
                13 => 51,
                14 => 57,
                15 => 63,
                16 => 70,
                17 => 90,
                18 => 130,
                19 => 200,
                20 => 280,
                _ => unreachable!(),
            },
            Self::None => 0,
        }
    }
}

impl Default for HuntersClothes {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum HuntingOutfit {
    HuntersClothesCostume,
    HuntersClothes,
    None,
}

impl HuntingOutfit {
    pub fn xp_boost(self) -> f32 {
        match self {
            Self::HuntersClothes | Self::HuntersClothesCostume => 10.0,
            Self::None => 0.0,
        }
    }
}

impl Default for HuntingOutfit {
    fn default() -> Self {
        Self::None
    }
}
