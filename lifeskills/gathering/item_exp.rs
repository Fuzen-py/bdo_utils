// TODO: DO something with this
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ItemBaseEXP {
    WildGrass,
    SunriseHerb,
    SilverAzalea,
    DryManeGrass,
    FireFlakeFlower,
    SilkHoneyGrass,
    Everlasting,
    HardBlackShard,
    SharpBlackShard,
    BlackGemFragment,
    Caphras,
    AncientSpiritDust,
    DeepBlueHoofRoot,
    LailaPetal,
}

impl ItemBaseEXP {
    fn to_exp(self) -> u32 {
        match self {
            Self::WildGrass => 300,
            Self::SunriseHerb => 200,
            Self::SilverAzalea => 200,
            Self::DryManeGrass => 200,
            Self::FireFlakeFlower => 200,
            Self::SilkHoneyGrass => 200,
            Self::Everlasting => 2000,
            Self::HardBlackShard => 2000,
            Self::SharpBlackShard => 3000,
            Self::BlackGemFragment => 300,
            Self::Caphras => 300,
            Self::DeepBlueHoofRoot => 1000,
            Self::FemaleKermes=>1000,
            Self::FlamingFemaleKermes=>1000,
            Self::FairyPowder=>100,
            Self::MassOfPureMagic=>3000,
            Self::Fruit=>300,
            Self::Meat=>300,
            Self::RoughStone=>120,
            Self::ZincOre=>200,
            Self::CopperOre=>200,
            Self::IronOre=>200,
            Self::MeltedZincShard=>200,
            Self::MeltedIronShard=>200,
            Self::MeltedCopperShard=>200,
            Self::SilverOre=>500,
            Self::GoldOre=>500,
            Self::PlatinumOre=>500,
            Self::Coal=>300,
            Self::Jewel=>1000,
            Self::RoughtDiamond=>1000,
            Self::RoughEmerald=>1000,
            Self::RoughTopaz=>1000,
            Self::RoughSapphire=>1000,
            Self::RoughRuby=>1000
        }
    }
}
