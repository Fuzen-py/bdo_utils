use std::str::FromStr;

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

#[derive(Debug, Default, Clone, Copy)]
pub struct LifeSkillLevel {
    pub grade: Grade,
    pub level: Level,
}
