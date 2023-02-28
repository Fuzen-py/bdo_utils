use serde::{Deserialize, Serialize};
pub mod lifeskills;
pub mod market;

// TODO: Write something to help reach the cheapest mastery target
// TODO: Finish the other lifeskills
// TODO: Store energy regen rate in
// TODO: Account for all forms of XP buffs
// TODO: Calculate the rate for more hunting items(?)
// TODO: Write a config for this
// TODO: Statistics for hunting kills/h
// TODO: Cooking optimization (what to make from scratch)
// TODO: Fix the awful design to send gear around

pub(crate) fn enhancement_display(lvl: u8, clamp: bool, roman: bool) -> &'static str {
    match lvl {
        0 => {
            if clamp {
                if roman {
                    "I"
                } else {
                    "PRI:"
                }
            } else {
                "+1"
            }
        }
        1 => {
            if clamp {
                if roman {
                    "II"
                } else {
                    "DUO:"
                }
            } else {
                "+2"
            }
        }
        3 => {
            if clamp {
                if roman {
                    "III"
                } else {
                    "TRI:"
                }
            } else {
                "+3"
            }
        }
        4 => {
            if clamp {
                if roman {
                    "IV"
                } else {
                    "TET:"
                }
            } else {
                "+4"
            }
        }
        5 => {
            if clamp {
                if roman {
                    "V"
                } else {
                    "PEN:"
                }
            } else {
                "+5"
            }
        }
        6 => "+6",
        7 => "+7",
        8 => "+8",
        9 => "+10",
        11 => "+11",
        12 => "+12",
        13 => "+13",
        14 => "+14",
        15 => "+15",
        16 => {
            if roman {
                "I"
            } else {
                "PRI:"
            }
        }
        17 => {
            if roman {
                "II"
            } else {
                "DUO:"
            }
        }
        18 => {
            if roman {
                "III"
            } else {
                "TRI:"
            }
        }
        19 => {
            if roman {
                "IV"
            } else {
                "TET:"
            }
        }
        20 => {
            if roman {
                "V"
            } else {
                "PEN:"
            }
        }
        _ => unreachable!(),
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Class {
    Warrior,
    Ranger,
    Sorceress,
    Berserker,
    Valkyrie,
    Wizard,
    Witch,
    Tamer,
    Maehwa,
    Musa,
    Ninja,
    Kuno,
    DarkKnight,
    Striker,
    Mystic,
    Lahn,
    Archer,
    Shai,
    Guardian,
    Hashashin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub class: Class,
    pub name: String,
    pub life_skills: lifeskills::LifeSkill,
    pub breath: u16,
    pub strength: u16,
    pub health: u16,
    pub level: u16,
}
