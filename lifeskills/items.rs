use crate::enhancement_display;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LifeskillAccessories {
    ManosNecklace { enhancement: u8 },
    ManosEarring { enhancement: u8 },
    ManosRing { enhancement: u8 },
    ManosBelt { enhancement: u8 },
    GeranoaNecklace { enhancement: u8 },
    GeranoaEarring { enhancement: u8 },
    GeranoaRing { enhancement: u8 },
    GeranoaBelt { enhancement: u8 },
    LoggiaNecklace { enhancement: u8 },
    LoggiaEarring { enhancement: u8 },
    LoggiaRing { enhancement: u8 },
    LoggiaBelt { enhancement: u8 },
    None,
}
macro_rules! pretty_accessory {
    ($f:expr, $acc:expr, $lvl:expr) => {
        write!($f, $acc, enhancement_display($lvl, true, false))
    };
}

impl fmt::Display for LifeskillAccessories {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ManosNecklace { enhancement } => {
                pretty_accessory!(f, "{} Manos Necklace", enhancement)
            }
            Self::ManosBelt { enhancement } => pretty_accessory!(f, "{} Manos Belt", enhancement),
            Self::ManosEarring { enhancement } => {
                pretty_accessory!(f, "{} Manos Earring", enhancement)
            }
            Self::ManosRing { enhancement } => pretty_accessory!(f, "{} Manos Ring", enhancement),
            Self::GeranoaBelt { enhancement } => {
                pretty_accessory!(f, "{} Geranoa Belt", enhancement)
            }
            Self::GeranoaEarring { enhancement } => {
                pretty_accessory!(f, "{} Geranoa Earring", enhancement)
            }
            Self::GeranoaNecklace { enhancement } => {
                pretty_accessory!(f, "{} Gernoa Necklace", enhancement)
            }
            Self::GeranoaRing { enhancement } => {
                pretty_accessory!(f, "{} Geranoa Ring", enhancement)
            }
            Self::LoggiaBelt { enhancement } => pretty_accessory!(f, "{} Loggia Belt", enhancement),
            Self::LoggiaEarring { enhancement } => {
                pretty_accessory!(f, "{} Loggia Earring", enhancement)
            }
            Self::LoggiaNecklace { enhancement } => {
                pretty_accessory!(f, "{} Loggia Necklace", enhancement)
            }
            Self::LoggiaRing { enhancement } => pretty_accessory!(f, "{} Loggia Ring", enhancement),
            Self::None => f.write_str("-"),
        }
    }
}

pub(crate) const MANOS_SET_EFFECT: f32 = 5.0;
pub(crate) const GERANOA_SET_EFFECT: f32 = 3.5;
pub(crate) const LOGGIA_SET_EFFECT: f32 = 2.0;

impl LifeskillAccessories {
    pub fn xp_bonus(self) -> f32 {
        match self {
            Self::ManosBelt { .. } | Self::None => 0.0,
            Self::ManosNecklace { enhancement }
            | Self::ManosRing { enhancement }
            | Self::ManosEarring { enhancement } => (2 * enhancement) as f32,
            Self::GeranoaBelt { .. } => 0.0,
            Self::GeranoaNecklace { enhancement }
            | Self::GeranoaEarring { enhancement }
            | Self::GeranoaRing { enhancement } => enhancement as f32,
            Self::LoggiaBelt { .. } => 0.0,
            Self::LoggiaNecklace { enhancement }
            | Self::LoggiaRing { enhancement }
            | Self::LoggiaEarring { enhancement } => (enhancement as f32 * 0.5).ceil() as f32,
        }
    }
    pub fn set_effect(self) -> f32 {
        match self {
            Self::ManosBelt { .. }
            | Self::ManosEarring { .. }
            | Self::ManosNecklace { .. }
            | Self::ManosRing { .. } => MANOS_SET_EFFECT,
            Self::GeranoaBelt { .. }
            | Self::GeranoaEarring { .. }
            | Self::GeranoaNecklace { .. }
            | Self::GeranoaRing { .. } => GERANOA_SET_EFFECT,
            Self::LoggiaBelt { .. }
            | Self::LoggiaEarring { .. }
            | Self::LoggiaNecklace { .. }
            | Self::LoggiaRing { .. } => LOGGIA_SET_EFFECT,
            Self::None => 0.0,
        }
    }
    pub fn is_manos(self) -> bool {
        match self {
            Self::ManosBelt { .. }
            | Self::ManosEarring { .. }
            | Self::ManosNecklace { .. }
            | Self::ManosRing { .. } => true,
            _ => false,
        }
    }
    pub fn is_loggia(self) -> bool {
        match self {
            Self::LoggiaBelt { .. }
            | Self::LoggiaEarring { .. }
            | Self::LoggiaNecklace { .. }
            | Self::LoggiaRing { .. } => true,
            _ => false,
        }
    }
    pub fn is_geranoa(self) -> bool {
        match self {
            Self::GeranoaBelt { .. }
            | Self::GeranoaEarring { .. }
            | Self::GeranoaNecklace { .. }
            | Self::GeranoaRing { .. } => true,
            _ => false,
        }
    }
    pub fn is_none(self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
    pub fn is_some(self) -> bool {
        !self.is_none()
    }
    pub fn mastery(self) -> u32 {
        match self {
            Self::ManosNecklace { enhancement } | Self::ManosBelt { enhancement } => {
                match enhancement {
                    0 => 15,
                    1 => 30,
                    2 => 50,
                    3 => 75,
                    4 => 105,
                    5 => 150,
                    _ => unreachable!(),
                }
            }
            Self::ManosEarring { enhancement } | Self::ManosRing { enhancement } => {
                match enhancement {
                    0 => 10,
                    1 => 25,
                    2 => 40,
                    3 => 65,
                    4 => 90,
                    5 => 125,
                    _ => unreachable!(),
                }
            }
            Self::GeranoaNecklace { enhancement } | Self::GeranoaBelt { enhancement } => {
                match enhancement {
                    0 => 8,
                    1 => 14,
                    2 => 26,
                    3 => 45,
                    4 => 70,
                    5 => 110,
                    _ => unreachable!(),
                }
            }
            Self::GeranoaEarring { enhancement } | Self::GeranoaRing { enhancement } => {
                match enhancement {
                    0 => 6,
                    1 => 12,
                    2 => 22,
                    3 => 36,
                    4 => 55,
                    5 => 95,
                    _ => unreachable!(),
                }
            }
            Self::LoggiaBelt { enhancement } | Self::LoggiaNecklace { enhancement } => {
                match enhancement {
                    0 => 5,
                    1 => 10,
                    2 => 18,
                    3 => 28,
                    4 => 40,
                    5 => 60,
                    _ => unreachable!(),
                }
            }
            Self::LoggiaEarring { enhancement } | Self::LoggiaRing { enhancement } => {
                match enhancement {
                    0 => 4,
                    1 => 8,
                    2 => 16,
                    3 => 24,
                    4 => 32,
                    5 => 50,
                    _ => unreachable!(),
                }
            }
            Self::None => 0,
        }
    }
}

impl Default for LifeskillAccessories {
    fn default() -> Self {
        Self::None
    }
}
