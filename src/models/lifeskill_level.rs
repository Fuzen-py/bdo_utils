pub enum Grade {
    Beginner,
    Apprentice,
    Skilled,
    Professional,
    Artisan,
    Master,
    Guru,
}

pub struct Level {
    level: u8,
    progress: f32,
}

pub struct LifeSkillLevel {
    pub grade: Grade,
    pub level: Level,
}

impl LifeSkillLevel {
    fn new(grade: Grade, level: Level) -> Self {
        // TODO: Set max's
        LifeSkillLevel { grade, level }
    }
}
