use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum Location {
    // Balenos Territory
    Velia,
    Olvia,
    IliyaIsland,
    // Serendia Territory
    Hidel,
    Glish,
    // Calpheon Territory
    Keplan,
    PortEpheria,
    Trent,
    // Media Territory
    Altinova,
    Tarif,
    // Valencia Territory
    ValenciaCity,
    Shakatu,
    SandGrainBazaar,
    AncadoInnerHarbor,
    ArehazaTown,
    Muiquun,
    // Margoria
    PortRatt,
    // Kamasylvia
    OldWisdomTree,
    Grana,
    // Dreihgan
    Duvencrune,
    // O'dyllita
    Odraxxia,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Location::*;
        match self {
            Velia => f.write_str("Velia"),
            Olvia => f.write_str("Olvia"),
            IliyaIsland => f.write_str("Iliya Island"),
            Hidel => f.write_str("Hidel"),
            Glish => f.write_str("Glish"),
            Keplan => f.write_str("Keplan"),
            PortEpheria => f.write_str("Port Epheria"),
            Trent => f.write_str("Trent"),
            Altinova => f.write_str("Altinova"),
            Tarif => f.write_str("Tarif"),
            ValenciaCity => f.write_str("Valencia City"),
            Shakatu => f.write_str("Shakatu"),
            SandGrainBazaar => f.write_str("Sand Grain Bazaar"),
            AncadoInnerHarbor => f.write_str("Ancado Inner Harbor"),
            ArehazaTown => f.write_str("Arehaza Town"),
            Muiquun => f.write_str("Muiquun"),
            PortRatt => f.write_str("Port Ratt"),
            OldWisdomTree => f.write_str("Old Wisdom Tree"),
            Grana => f.write_str("Grána"),
            Duvencrune => f.write_str("Duvencrune"),
            Odraxxia => f.write_str("O'draxxia"),
        }
    }
}
