use std::str::FromStr;

pub(crate) mod XP;

#[inline]
fn adjust(s: &mut LifeSkillLevel, max: u8, next_grade: Grade) {
    s.level.correct();
    if s.level.level >= max {
        s.level.level -= max;
        s.grade = next_grade;
        s.correct();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct LifeskillXP(u64);

#[derive(Debug, Clone, Copy)]
pub enum Grade {
    Beginner,
    Apprentice,
    Skilled,
    Professional,
    Artisan,
    Master,
    Guru,
}

impl FromStr for Grade {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let txt = s.trim().to_ascii_lowercase();
        Ok(match txt.as_str() {
            "beginner" => Grade::Beginner,
            "apprentice" => Grade::Apprentice,
            "skilled" => Grade::Skilled,
            "professional" => Grade::Professional,
            "artisan" => Grade::Artisan,
            "Master" => Grade::Master,
            "Guru" => Grade::Guru,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("{} is not a supported grade", txt),
                ))
            }
        })
    }
}

impl Default for Grade {
    fn default() -> Self {
        Grade::Beginner
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Level {
    pub level: u8,
    pub progress: f32,
}

impl Level {
    fn correct(&mut self) {
        if self.progress >= 100.0 {
            let lvls = (self.progress / 100.0).floor() as u8;
            self.level += lvls;
            self.progress -= lvls as f32 * 100.0;
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LifeSkillLevel {
    pub grade: Grade,
    pub level: Level,
}

impl LifeSkillLevel {
    /// Correct Level Offsets
    pub fn correct(&mut self) {
        match self.grade {
            Grade::Beginner => adjust(self, 10, Grade::Apprentice),
            Grade::Apprentice => adjust(self, 10, Grade::Skilled),
            Grade::Skilled => adjust(self, 10, Grade::Professional),
            Grade::Professional => adjust(self, 10, Grade::Artisan),
            Grade::Artisan => adjust(self, 10, Grade::Master),
            Grade::Master => adjust(self, 30, Grade::Guru),
            Grade::Guru => self.level.correct(),
        }
    }

    const fn initial_total_xp_to_next(&self) -> u64 {
        match self.grade {
            Grade::Beginner => match self.level.level as usize {
                lvl if lvl < 10 => XP::BEGINNER[lvl] - XP::BEGINNER[lvl - 1],
                _ => XP::APPRENTICE[0] - XP::BEGINNER[9],
            },
            Grade::Apprentice => match self.level.level as usize {
                lvl if lvl < 10 => XP::APPRENTICE[lvl] - XP::APPRENTICE[lvl - 1],
                _ => XP::SKILLED[0] - XP::APPRENTICE[9],
            },
            Grade::Skilled => match self.level.level as usize {
                lvl if lvl < 10 => XP::SKILLED[lvl] - XP::SKILLED[lvl - 1],
                _ => XP::PROFESSIONAL[0] - XP::SKILLED[9],
            },
            Grade::Professional => match self.level.level as usize {
                lvl if lvl < 10 => XP::PROFESSIONAL[lvl] - XP::PROFESSIONAL[lvl - 1],
                _ => XP::ARTISAN[0] - XP::PROFESSIONAL[9],
            },
            Grade::Artisan => match self.level.level as usize {
                lvl if lvl < 10 => XP::ARTISAN[lvl] - XP::ARTISAN[lvl - 1],
                _ => XP::MASTER[0] - XP::ARTISAN[9],
            },
            Grade::Master => match self.level.level as usize {
                lvl if lvl < 30 => XP::MASTER[lvl] - XP::MASTER[lvl - 1],
                _ => XP::GURU[0] - XP::MASTER[29],
            },
            Grade::Guru => match self.level.level as usize {
                lvl if lvl < 50 => XP::GURU[lvl] - XP::GURU[lvl - 1],
                _ => XP::GURU[50] - XP::GURU[49],
            },
        }
    }

    pub(crate) fn total_xp_to_next(self) -> u64 {
        let xp = self.initial_total_xp_to_next();
        (xp as f64 - (xp as f64 * (self.level.progress / 100.0) as f64)).floor() as u64
    }

    pub(crate) fn rank_total_xp(self) -> u64 {
        let lvl = self.level.level as usize;
        // TODO: Validate & verify its the same for all lifeskills
        match self.grade {
            Grade::Beginner => XP::BEGINNER[lvl],
            Grade::Apprentice => XP::APPRENTICE[lvl],
            Grade::Skilled => XP::SKILLED[lvl],
            Grade::Professional => XP::PROFESSIONAL[lvl],
            Grade::Artisan => XP::ARTISAN[lvl],
            Grade::Master => XP::MASTER[lvl],
            Grade::Guru => XP::GURU[lvl],
        }
    }
}
impl From<LifeSkillLevel> for LifeskillXP {
    fn from(val: LifeSkillLevel) -> Self {
        LifeskillXP(val.total_xp_to_next() + val.rank_total_xp())
    }
}

impl From<LifeskillXP> for LifeSkillLevel {
    fn from(val: LifeskillXP) -> Self {
        let LifeskillXP(xp) = val;
        let mut level = xp_to_lifelvl(val);
        let total_xp = level.rank_total_xp();
        let xp = total_xp - xp;
        level.level.progress = ((xp as f64) / total_xp as f64) as f32 * 100.0;
        level
    }
}

macro_rules! life {
    ($grade:expr, $level:literal) => {
        LifeSkillLevel {
            grade: $grade,
            level: Level {
                level: $level,
                progress: 0.0,
            },
        }
    };
    ($grade:expr, $level:literal, $progress:literal) => {
        LifeSkillLevel {
            grade: $grade,
            level: Level {
                level: $level,
                progress: $progress,
            },
        }
    };
}
// Maybe change this to (Grade, Level, Additional XP)
// Consts cant divide which is needed for progress
// BLOCKED: https://github.com/rust-lang/rust/issues/57241
/// Convert XP to lifeskill level
pub fn xp_to_lifelvl(lxp: LifeskillXP) -> LifeSkillLevel {
    let LifeskillXP(xp) = lxp;
    match xp {
        xp if xp < XP::BEGINNER[0] => life!(Grade::Beginner, 1),
        xp if xp < XP::BEGINNER[1] => life!(Grade::Beginner, 2),
        xp if xp < XP::BEGINNER[2] => life!(Grade::Beginner, 3),
        xp if xp < XP::BEGINNER[3] => life!(Grade::Beginner, 4),
        xp if xp < XP::BEGINNER[4] => life!(Grade::Beginner, 5),
        xp if xp < XP::BEGINNER[5] => life!(Grade::Beginner, 6),
        xp if xp < XP::BEGINNER[6] => life!(Grade::Beginner, 7),
        xp if xp < XP::BEGINNER[7] => life!(Grade::Beginner, 8),
        xp if xp < XP::BEGINNER[8] => life!(Grade::Beginner, 9),
        xp if xp < XP::BEGINNER[9] => life!(Grade::Beginner, 10),
        xp if xp < XP::APPRENTICE[0] => life!(Grade::Apprentice, 1),
        xp if xp < XP::APPRENTICE[1] => life!(Grade::Apprentice, 2),
        xp if xp < XP::APPRENTICE[2] => life!(Grade::Apprentice, 3),
        xp if xp < XP::APPRENTICE[3] => life!(Grade::Apprentice, 4),
        xp if xp < XP::APPRENTICE[4] => life!(Grade::Apprentice, 5),
        xp if xp < XP::APPRENTICE[5] => life!(Grade::Apprentice, 6),
        xp if xp < XP::APPRENTICE[6] => life!(Grade::Apprentice, 7),
        xp if xp < XP::APPRENTICE[7] => life!(Grade::Apprentice, 8),
        xp if xp < XP::APPRENTICE[8] => life!(Grade::Apprentice, 9),
        xp if xp < XP::APPRENTICE[9] => life!(Grade::Apprentice, 10),
        xp if xp < XP::SKILLED[0] => life!(Grade::Skilled, 1),
        xp if xp < XP::SKILLED[1] => life!(Grade::Skilled, 2),
        xp if xp < XP::SKILLED[2] => life!(Grade::Skilled, 3),
        xp if xp < XP::SKILLED[3] => life!(Grade::Skilled, 4),
        xp if xp < XP::SKILLED[4] => life!(Grade::Skilled, 5),
        xp if xp < XP::SKILLED[5] => life!(Grade::Skilled, 6),
        xp if xp < XP::SKILLED[6] => life!(Grade::Skilled, 7),
        xp if xp < XP::SKILLED[7] => life!(Grade::Skilled, 8),
        xp if xp < XP::SKILLED[8] => life!(Grade::Skilled, 9),
        xp if xp < XP::SKILLED[9] => life!(Grade::Skilled, 10),
        xp if xp < XP::PROFESSIONAL[0] => life!(Grade::Professional, 1),
        xp if xp < XP::PROFESSIONAL[1] => life!(Grade::Professional, 2),
        xp if xp < XP::PROFESSIONAL[2] => life!(Grade::Professional, 3),
        xp if xp < XP::PROFESSIONAL[3] => life!(Grade::Professional, 4),
        xp if xp < XP::PROFESSIONAL[4] => life!(Grade::Professional, 5),
        xp if xp < XP::PROFESSIONAL[5] => life!(Grade::Professional, 6),
        xp if xp < XP::PROFESSIONAL[6] => life!(Grade::Professional, 7),
        xp if xp < XP::PROFESSIONAL[7] => life!(Grade::Professional, 8),
        xp if xp < XP::PROFESSIONAL[8] => life!(Grade::Professional, 9),
        xp if xp < XP::PROFESSIONAL[9] => life!(Grade::Professional, 10),
        XP if xp < XP::ARTISAN[0] => life!(Grade::Artisan, 1),
        xp if xp < XP::ARTISAN[1] => life!(Grade::Artisan, 2),
        xp if xp < XP::ARTISAN[2] => life!(Grade::Artisan, 3),
        xp if xp < XP::ARTISAN[3] => life!(Grade::Artisan, 4),
        xp if xp < XP::ARTISAN[4] => life!(Grade::Artisan, 5),
        xp if xp < XP::ARTISAN[5] => life!(Grade::Artisan, 6),
        xp if xp < XP::ARTISAN[6] => life!(Grade::Artisan, 7),
        xp if xp < XP::ARTISAN[7] => life!(Grade::Artisan, 8),
        xp if xp < XP::ARTISAN[8] => life!(Grade::Artisan, 9),
        xp if xp < XP::ARTISAN[9] => life!(Grade::Artisan, 10),
        xp if xp < XP::MASTER[0] => life!(Grade::Master, 1),
        xp if xp < XP::MASTER[1] => life!(Grade::Master, 2),
        xp if xp < XP::MASTER[2] => life!(Grade::Master, 3),
        xp if xp < XP::MASTER[3] => life!(Grade::Master, 4),
        xp if xp < XP::MASTER[4] => life!(Grade::Master, 5),
        xp if xp < XP::MASTER[5] => life!(Grade::Master, 6),
        xp if xp < XP::MASTER[6] => life!(Grade::Master, 7),
        xp if xp < XP::MASTER[7] => life!(Grade::Master, 8),
        xp if xp < XP::MASTER[8] => life!(Grade::Master, 9),
        xp if xp < XP::MASTER[9] => life!(Grade::Master, 10),
        xp if xp < XP::MASTER[10] => life!(Grade::Master, 11),
        xp if xp < XP::MASTER[11] => life!(Grade::Master, 12),
        xp if xp < XP::MASTER[12] => life!(Grade::Master, 13),
        xp if xp < XP::MASTER[13] => life!(Grade::Master, 14),
        xp if xp < XP::MASTER[14] => life!(Grade::Master, 15),
        xp if xp < XP::MASTER[15] => life!(Grade::Master, 16),
        xp if xp < XP::MASTER[16] => life!(Grade::Master, 17),
        xp if xp < XP::MASTER[17] => life!(Grade::Master, 18),
        xp if xp < XP::MASTER[18] => life!(Grade::Master, 19),
        xp if xp < XP::MASTER[19] => life!(Grade::Master, 20),
        xp if xp < XP::MASTER[20] => life!(Grade::Master, 21),
        xp if xp < XP::MASTER[21] => life!(Grade::Master, 22),
        xp if xp < XP::MASTER[22] => life!(Grade::Master, 23),
        xp if xp < XP::MASTER[23] => life!(Grade::Master, 24),
        xp if xp < XP::MASTER[24] => life!(Grade::Master, 25),
        xp if xp < XP::MASTER[25] => life!(Grade::Master, 26),
        xp if xp < XP::MASTER[26] => life!(Grade::Master, 27),
        xp if xp < XP::MASTER[27] => life!(Grade::Master, 28),
        xp if xp < XP::MASTER[28] => life!(Grade::Master, 29),
        xp if xp < XP::MASTER[29] => life!(Grade::Master, 30),
        xp if xp < XP::GURU[0] => life!(Grade::Guru, 1),
        xp if xp < XP::GURU[1] => life!(Grade::Guru, 2),
        xp if xp < XP::GURU[2] => life!(Grade::Guru, 3),
        xp if xp < XP::GURU[3] => life!(Grade::Guru, 4),
        xp if xp < XP::GURU[4] => life!(Grade::Guru, 5),
        xp if xp < XP::GURU[5] => life!(Grade::Guru, 6),
        xp if xp < XP::GURU[6] => life!(Grade::Guru, 7),
        xp if xp < XP::GURU[7] => life!(Grade::Guru, 8),
        xp if xp < XP::GURU[8] => life!(Grade::Guru, 9),
        xp if xp < XP::GURU[9] => life!(Grade::Guru, 10),
        xp if xp < XP::GURU[10] => life!(Grade::Guru, 11),
        xp if xp < XP::GURU[11] => life!(Grade::Guru, 12),
        xp if xp < XP::GURU[12] => life!(Grade::Guru, 13),
        xp if xp < XP::GURU[13] => life!(Grade::Guru, 14),
        xp if xp < XP::GURU[14] => life!(Grade::Guru, 15),
        xp if xp < XP::GURU[15] => life!(Grade::Guru, 16),
        xp if xp < XP::GURU[16] => life!(Grade::Guru, 17),
        xp if xp < XP::GURU[17] => life!(Grade::Guru, 18),
        xp if xp < XP::GURU[18] => life!(Grade::Guru, 19),
        xp if xp < XP::GURU[19] => life!(Grade::Guru, 20),
        xp if xp < XP::GURU[20] => life!(Grade::Guru, 21),
        xp if xp < XP::GURU[21] => life!(Grade::Guru, 22),
        xp if xp < XP::GURU[22] => life!(Grade::Guru, 23),
        xp if xp < XP::GURU[23] => life!(Grade::Guru, 24),
        xp if xp < XP::GURU[24] => life!(Grade::Guru, 25),
        xp if xp < XP::GURU[25] => life!(Grade::Guru, 26),
        xp if xp < XP::GURU[26] => life!(Grade::Guru, 27),
        xp if xp < XP::GURU[27] => life!(Grade::Guru, 28),
        xp if xp < XP::GURU[28] => life!(Grade::Guru, 29),
        xp if xp < XP::GURU[29] => life!(Grade::Guru, 30),
        xp if xp < XP::GURU[30] => life!(Grade::Guru, 31),
        xp if xp < XP::GURU[31] => life!(Grade::Guru, 32),
        xp if xp < XP::GURU[32] => life!(Grade::Guru, 33),
        xp if xp < XP::GURU[33] => life!(Grade::Guru, 34),
        xp if xp < XP::GURU[34] => life!(Grade::Guru, 35),
        xp if xp < XP::GURU[35] => life!(Grade::Guru, 36),
        xp if xp < XP::GURU[36] => life!(Grade::Guru, 37),
        xp if xp < XP::GURU[37] => life!(Grade::Guru, 38),
        xp if xp < XP::GURU[38] => life!(Grade::Guru, 39),
        xp if xp < XP::GURU[39] => life!(Grade::Guru, 40),
        xp if xp < XP::GURU[40] => life!(Grade::Guru, 41),
        xp if xp < XP::GURU[41] => life!(Grade::Guru, 42),
        xp if xp < XP::GURU[42] => life!(Grade::Guru, 43),
        xp if xp < XP::GURU[43] => life!(Grade::Guru, 44),
        xp if xp < XP::GURU[44] => life!(Grade::Guru, 45),
        xp if xp < XP::GURU[45] => life!(Grade::Guru, 46),
        xp if xp < XP::GURU[46] => life!(Grade::Guru, 47),
        xp if xp < XP::GURU[47] => life!(Grade::Guru, 48),
        xp if xp < XP::GURU[48] => life!(Grade::Guru, 49),
        xp if xp < XP::GURU[49] => life!(Grade::Guru, 50),
        _ => life!(Grade::Guru, 50, 99.9),
    }
}
