use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Class {
    Warrior,
    Ranger,
    Sorceress,
    Berserker,
    Tamer,
    Musa,
    Maehwa,
    Valkyrie,
    Kunoichi,
    Ninja,
    Wizzard,
    Witch,
    Mystic,
    Striker,
    Lahn,
    Archer,
    DarkKnight,
    Shai,
    Guardian,
    Hashashin,
    Nova,
}