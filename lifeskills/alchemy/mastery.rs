use serde::{Deserialize, Serialize};

const AlchemyBrackets: [u32; 40] = [
    0, 50, 100, 150, 200, 250, 300, 400, 450, 500, 550, 600, 650, 700, 750, 800, 850, 900, 950,
    1000, 1050, 1100, 1150, 1200, 1250, 1300, 1350, 1400, 1450, 1500, 1550, 1600, 1650, 1700, 1750,
    1800, 1850, 1900, 1950, 2000,
];

macro_rules! mastery {
    ($($masterylvl:expr, $regular:expr, $special:expr, $rare:expr, $imperial:expr;)) => ([
        $(
            MasteryBonus {
                mastery: $masterylvl,
                proc_chance: ProcChance {
                    regular: $regular,
                    special: $special,
                    rare: $rare,
                },
                imperial_bonus: $imperial
            },
        )
    ])
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct MasteryBonus {
    pub mastery: u32,
    pub proc_chance: ProcChance,
    pub imperial_bonus: f32,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ProcChance {
    pub regular: f32,
    pub special: f32,
    pub rare: f32,
}

impl MasteryBonus {
    pub fn get_bonus(mastery: u32) -> Self {
        let mut m: Vec<[u32; 2]> = AlchemyBrackets
            .iter()
            .filter_map(|m| {
                let bracket = m;
                m.checked_sub(mastery).and_then(|nm| Some([nm, *bracket]))
            })
            .collect();
        m.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
        let bracket = m.first().unwrap()[1];
        match bracket {
            0 => mastery! { 0, 0.00, 0.00, 0.00, 0.00, 0.00 },
            50 => mastery! { 50, 0.0576, 0.0028, 0.0005, 0.0001, 0.027 },
        }
    }
}
