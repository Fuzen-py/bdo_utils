macro_rules! clamp {
    ($val:expr, $max:expr, $min:expr) => {
        if $val <= $max {
            $val
        } else if $val < $min {
            $min
        } else {
            $val
        }
    };
}

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub enum MasteryLevel {
    Beginner { level: u8, xp: f32 },     // 10 Max
    Apprentice { level: u8, xp: f32 },   // 10 Max
    Skilled { level: u8, xp: f32 },      // 10 Max
    Professional { level: u8, xp: f32 }, // 10 Max
    Artisan { level: u8, xp: f32 },      // 10 Max
    Master { level: u8, xp: f32 },       // 30 Max
    Guru { level: u8, xp: f32 },         // 50 Max
}

impl MasteryLevel {
    pub const fn max() -> Self {
        Self::Guru {
            level: 51,
            xp: 99.99,
        }
    }
    pub const fn min() -> Self {
        Self::Beginner { level: 1, xp: 0.0 }
    }
    #[allow(clippy::cognitive_complexity)]
    pub fn correct(self) -> Self {
        match self {
            Self::Beginner { level, xp } => Self::Beginner {
                level: clamp!(level, 10, 1),
                xp: clamp!(xp, 99.99, 0.0),
            },
            Self::Apprentice { level, xp } => Self::Apprentice {
                level: clamp!(level, 10, 1),
                xp: clamp!(xp, 99.99, 0.0),
            },
            Self::Skilled { level, xp } => Self::Skilled {
                level: clamp!(level, 10, 1),
                xp: clamp!(xp, 99.99, 0.0),
            },
            Self::Professional { level, xp } => Self::Professional {
                level: clamp!(level, 10, 1),
                xp: clamp!(xp, 99.99, 0.0),
            },
            Self::Artisan { level, xp } => Self::Artisan {
                level: clamp!(level, 10, 1),
                xp: clamp!(xp, 99.99, 0.0),
            },
            Self::Master { level, xp } => Self::Master {
                level: clamp!(level, 30, 1),
                xp: clamp!(xp, 99.99, 0.0),
            },
            Self::Guru { level, xp } => Self::Guru {
                level: clamp!(level, 51, 1),
                xp: clamp!(xp, 99.99, 0.0),
            },
        }
    }
    pub fn mastery(self) -> u32 {
        match self {
            Self::Apprentice { level, .. } => (level * 5) as u32,
            Self::Beginner { level, .. } => 50 + (level * 5) as u32,
            Self::Skilled { level, .. } => 100 + (level * 5) as u32,
            Self::Professional { level, .. } => 150 + (level * 5) as u32,
            Self::Artisan { level, .. } => 200 + (level * 5) as u32,
            Self::Master { level, .. } => 250 + (level * 5) as u32,
            Self::Guru { level, .. } => 400 + (level * 10) as u32,
        }
    }
    pub(crate) fn total_xp_to_next(self) -> u64 {
        // TODO: Validate & verify its the same for all lifeskills
        match self {
            Self::Beginner { level, .. } => match level {
                1 => 400,
                2 => 600,
                3 => 1_200,
                4 => 1_900,
                5 => 2_900,
                6 => 4_000,
                7 => 5_400,
                8 => 6_900,
                9 => 8_600,
                10 => 10_600,
                _ => unreachable!(),
            },
            Self::Apprentice { level, .. } => match level {
                1 => 13_100,
                2 => 15_900,
                3 => 18_800,
                4 => 22_000,
                5 => 25_400,
                6 => 29_100,
                7 => 33_000,
                8 => 37_100,
                9 => 41_500,
                10 => 46_100,
                _ => unreachable!(),
            },
            Self::Skilled { level, .. } => match level {
                1 => 52_300,
                2 => 58_800,
                3 => 65_600,
                4 => 72_700,
                5 => 80_100,
                6 => 87_800,
                7 => 95_800,
                8 => 104_100,
                9 => 117_400,
                10 => 131_200,
                _ => unreachable!(),
            },
            Self::Professional { level, .. } => match level {
                1 => 150_200,
                2 => 174_600,
                3 => 204_600,
                4 => 245_100,
                5 => 296_500,
                6 => 363_900,
                7 => 452_600,
                8 => 568_100,
                9 => 720_800,
                10 => 873_600,
                _ => unreachable!(),
            },
            Self::Artisan { level, .. } => match level {
                1 => 1_074_400,
                2 => 1_339_100,
                3 => 1_641_000,
                4 => 1_990_900,
                5 => 2_452_600,
                6 => 3_015_300,
                7 => 3_663_100,
                8 => 4_470_600,
                9 => 5_490_800,
                10 => 6_511_100,
                _ => unreachable!(),
            },
            Self::Master { level, .. } => match level {
                1 => 7_536_500,
                2 => 8_567_100,
                3 => 9_603_000,
                4 => 10_644_300,
                5 => 11_691_100,
                6 => 12_743_500,
                7 => 13_801_600,
                8 => 14_865_500,
                9 => 15_935_300,
                10 => 17_011_100,
                11 => 18_093_000,
                12 => 19_181_100,
                13 => 20_275_500,
                14 => 21_376_300,
                15 => 22_483_600,
                16 => 23_597_500,
                17 => 24_718_100,
                18 => 25_854_500,
                19 => 26_979_800,
                20 => 28_121_100,
                21 => 29_269_500,
                22 => 30_425_100,
                23 => 31_588_000,
                24 => 32_758_300,
                25 => 33_936_100,
                26 => 35_121_500,
                27 => 36_314_600,
                28 => 37_515_500,
                29 => 38_724_300,
                30 => 39_941_100,
                _ => unreachable!(),
            },
            Self::Guru { level, .. } => match level {
                1 => 41_166_000,
                2 => 42_399_100,
                3 => 44_890_300,
                4 => 44_890_300,
                5 => 46_148_600,
                6 => 47_415_500,
                7 => 48_691_100,
                8 => 49_975_500,
                9 => 51_268_800,
                10 => 52_571_100,
                11 => 53_882_500,
                12 => 55_203_100,
                13 => 56_533_000,
                14 => 57_862_300,
                15 => 59_221_100,
                16 => 60_579_500,
                17 => 61_947_600,
                18 => 63_325_500,
                19 => 64_713_300,
                20 => 66_101_100,
                21 => 74_694_200,
                22 => 84_404_400,
                23 => 95_377_000,
                24 => 107_776_000,
                25 => 121_786_900,
                26 => 137_619_200,
                27 => 155_509_700,
                28 => 175_726_000,
                29 => 198_570_400,
                30 => 224_384_600,
                31 => 253_554_600,
                32 => 286_516_700,
                33 => 323_763_900,
                34 => 365_853_200,
                35 => 413_414_100,
                36 => 467_157_900,
                37 => 526_888_400,
                38 => 596_513_900,
                39 => 674_060_700,
                40 => 761_688_600,
                41 => 860_708_100,
                42 => 972_600_200,
                43 => 1_099_038_200,
                44 => 1_241_913_200,
                45 => 1_403_361_900,
                46 => 1_585_978_900,
                47 => 1_792_952_800,
                48 => 2_024_906_700,
                49 => 2_288_144_600,
                50 | 51 => 2_585_603_400,
                _ => unreachable!(),
            },
        }
    }
    pub(crate) fn rank_total_xp(self) -> u64 {
        // TODO: Validate & verify its the same for all lifeskills
        match self {
            Self::Beginner { level, .. } => match level {
                1 => 100,
                2 => 500,
                3 => 1_100,
                4 => 2_300,
                5 => 4_200,
                6 => 7_100,
                7 => 11_100,
                8 => 16_500,
                9 => 23_400,
                10 => 32_00,
                _ => unreachable!(),
            },
            Self::Apprentice { level, .. } => match level {
                1 => 42_600,
                2 => 55_700,
                3 => 71_600,
                4 => 90_400,
                5 => 112_400,
                6 => 137_800,
                7 => 166_900,
                8 => 199_900,
                9 => 237_000,
                10 => 278_500,
                _ => unreachable!(),
            },
            Self::Skilled { level, .. } => match level {
                1 => 324_600,
                2 => 376_900,
                3 => 435_700,
                4 => 501_300,
                5 => 574_000,
                6 => 654_100,
                7 => 741_900,
                8 => 837_700,
                9 => 941_800,
                10 => 1_059_200,
                _ => unreachable!(),
            },
            Self::Professional { level, .. } => match level {
                1 => 1_190_400,
                2 => 1_340_600,
                3 => 1_515_200,
                4 => 1_719_800,
                5 => 1_964_900,
                6 => 2_261_400,
                7 => 2_625_300,
                8 => 3_077_900,
                9 => 3_646_000,
                10 => 4_366_800,
                _ => unreachable!(),
            },
            Self::Artisan { level, .. } => match level {
                1 => 5_240_400,
                2 => 6_314_800,
                3 => 7_653_900,
                4 => 9_294_900,
                5 => 11_285_800,
                6 => 13_738_400,
                7 => 16_753_700,
                8 => 20_416_800,
                9 => 24_887_400,
                10 => 30_378_200,
                _ => unreachable!(),
            },
            Self::Master { level, .. } => match level {
                1 => 36_889_300,
                2 => 44_425_800,
                3 => 52_992_900,
                4 => 62_595_900,
                5 => 73_240_200,
                6 => 84_931_300,
                7 => 97_674_800,
                8 => 111_476_400,
                9 => 126_341_900,
                10 => 142_277_200,
                11 => 159_288_300,
                12 => 177_381_300,
                13 => 196_562_400,
                14 => 216_837_900,
                15 => 238_214_200,
                16 => 260_697_800,
                17 => 284_295_300,
                18 => 309_013_400,
                19 => 334_858_900,
                20 => 361_838_700,
                21 => 389_959_800,
                22 => 410_229_300,
                23 => 449_954_400,
                24 => 481_242_400,
                25 => 514_000_700,
                26 => 547_936_800,
                27 => 573_058_300,
                28 => 619_372_900,
                29 => 656_888_400,
                30 => 695_612_700,
                _ => unreachable!(),
            },
            Self::Guru { level, .. } => match level {
                1 => 735_553_800,
                2 => 776_719_800,
                3 => 819_118_900,
                4 => 862_759_400,
                5 => 907_649_700,
                6 => 953_798_300,
                7 => 1_001_213_800,
                8 => 1_049_904_900,
                9 => 1_099_880_400,
                10 => 1_151_149_200,
                11 => 1_203_720_300,
                12 => 1_257_602_800,
                13 => 1_312_805_900,
                14 => 1_369_338_900,
                15 => 1_427_211_200,
                16 => 1_486_432_300,
                17 => 1_547_011_800,
                18 => 1_608_959_400,
                19 => 1_672_284_900,
                20 => 1_736_998_200,
                21 => 1_803_099_300,
                22 => 1_877_793_500,
                23 => 1_962_197_900,
                24 => 2_057_574_900,
                25 => 2_165_350_900,
                26 => 2_287_137_800,
                27 => 2_424_757_000,
                28 => 2_580_266_700,
                29 => 2_755_992_700,
                30 => 2_954_563_100,
                31 => 3_178_947_700,
                32 => 3_432_502_300,
                33 => 3_719_019_000,
                34 => 4_042_782_900,
                35 => 4_408_636_100,
                36 => 4_822_050_200,
                37 => 5_289_208_100,
                38 => 5_817_096_500,
                39 => 6_413_610_400,
                40 => 7_087_671_100,
                41 => 7_849_359_700,
                42 => 8_710_067_800,
                43 => 9_682_668_000,
                44 => 10_781_706_200,
                45 => 12_023_619_400,
                46 => 13_426_981_300,
                47 => 15_012_780_200,
                48 => 16_804_733_000,
                49 => 18_829_639_700,
                50 => 21_117_784_300,
                51 => 23_703_387_700,
                _ => unreachable!(),
            },
        }
    }
    pub fn xp_to(self, goal: Self) -> u64 {
        goal.total_xp() - self.total_xp()
    }

    // Util to see if the diff is only XP
    pub(crate) fn is_xp_inc(self, rhs: Self) -> bool {
        if self == rhs {
            return false;
        }
        match self {
            Self::Beginner { level, .. } => {
                let lhslvl = level;
                match rhs {
                    Self::Beginner { level, .. } => lhslvl == level,
                    _ => false,
                }
            }
            Self::Apprentice { level, .. } => {
                let lhslvl = level;
                match rhs {
                    Self::Apprentice { level, .. } => lhslvl == level,
                    _ => false,
                }
            }
            Self::Skilled { level, .. } => {
                let lhslevel = level;
                match rhs {
                    Self::Skilled { level, .. } => lhslevel == level,
                    _ => false,
                }
            }
            Self::Professional { level, .. } => {
                let lhslvl = level;
                match rhs {
                    Self::Professional { level, .. } => lhslvl == level,
                    _ => false,
                }
            }
            Self::Artisan { level, .. } => {
                let lhslvl = level;
                match rhs {
                    Self::Artisan { level, .. } => lhslvl == level,
                    _ => false,
                }
            }
            Self::Master { level, .. } => {
                let lhslvl = level;
                match rhs {
                    Self::Master { level, .. } => lhslvl == level,
                    _ => false,
                }
            }
            Self::Guru { level, .. } => {
                let lhslvl = level;
                match rhs {
                    Self::Master { level, .. } => lhslvl == level,
                    _ => false,
                }
            }
        }
    }

    pub fn xp_to_next(self) -> u64 {
        let mut s = self;
        s.next().map(|n| self.xp_to(n)).unwrap_or_default()
    }
    pub fn total_xp(self) -> u64 {
        let prog = match self {
            Self::Beginner { xp, .. }
            | Self::Apprentice { xp, .. }
            | Self::Skilled { xp, .. }
            | Self::Professional { xp, .. }
            | Self::Artisan { xp, .. }
            | Self::Master { xp, .. }
            | Self::Guru { xp, .. } => xp,
        } / 100.0;
        self.rank_total_xp() + (self.total_xp_to_next() as f32 * prog).floor() as u64
    }
}

impl std::iter::Iterator for MasteryLevel {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        *self = match *self {
            Self::Beginner { level, .. } => match level {
                n @ 1..=9 => Self::Beginner {
                    level: n + 1,
                    xp: 0.0,
                },
                _ => Self::Apprentice { level: 1, xp: 0.0 },
            },
            Self::Apprentice { level, .. } => match level {
                n @ 1..=9 => Self::Apprentice {
                    level: n + 1,
                    xp: 0.0,
                },
                _ => Self::Apprentice { level: 1, xp: 0.0 },
            },
            Self::Skilled { level, .. } => match level {
                n @ 1..=9 => Self::Skilled {
                    level: n + 1,
                    xp: 0.0,
                },
                _ => Self::Professional { level: 0, xp: 0.0 },
            },
            Self::Professional { level, .. } => match level {
                n @ 1..=9 => Self::Professional {
                    level: n + 1,
                    xp: 0.0,
                },
                _ => Self::Artisan { level: 1, xp: 0.0 },
            },
            Self::Artisan { level, .. } => match level {
                n @ 1..=9 => Self::Artisan {
                    level: n + 1,
                    xp: 0.0,
                },
                _ => Self::Master { level: 1, xp: 0.0 },
            },
            Self::Master { level, .. } => match level {
                n @ 1..=29 => Self::Master {
                    level: n + 1,
                    xp: 0.0,
                },
                _ => Self::Guru { level: 1, xp: 0.0 },
            },
            Self::Guru { level, xp } => match level {
                n @ 1..=50 => Self::Guru {
                    level: n + 1,
                    xp: 0.0,
                },
                51 if xp < 99.99 => Self::Guru {
                    level: 51,
                    xp: 99.99,
                },
                _ => return None,
            },
        };
        Some(*self)
    }
}

impl Default for MasteryLevel {
    fn default() -> MasteryLevel {
        Self::min()
    }
}
