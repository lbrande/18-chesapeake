use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum ColorId {
    Yellow,
    Green,
    Brown,
    Gray,
    Red,
}

impl FromStr for ColorId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Yellow" => Ok(ColorId::Yellow),
            "Green" => Ok(ColorId::Green),
            "Brown" => Ok(ColorId::Brown),
            "Gray" => Ok(ColorId::Gray),
            "Red" => Ok(ColorId::Red),
            _ => Err(format!("{} can not be parsed as ColorId", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PubComId {
    BAndO,
    CAndA,
    CAndO,
    LV,
    NAndW,
    PRR,
    PAndLE,
    SRR,
}

impl PubComId {
    pub fn get_name(&self) -> &str {
        match self {
            PubComId::BAndO => "Baltimore and Ohio Railroad",
            PubComId::CAndA => "Camden and Amboy Railroad",
            PubComId::CAndO => "Chesapeake and Ohio Railroad",
            PubComId::LV => "Lehigh Valley Railroad",
            PubComId::NAndW => "Norfolk and Western Railway",
            PubComId::PRR => "Pennsylvania Railroad",
            PubComId::PAndLE => "Pittsburgh & Lake Erie Railroad",
            PubComId::SRR => "Strasburg Rail Road",
        }
    }
}

impl Display for PubComId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PubComId::BAndO => write!(f, "B&O"),
            PubComId::CAndA => write!(f, "C&A"),
            PubComId::CAndO => write!(f, "C&O"),
            PubComId::LV => write!(f, "LV"),
            PubComId::NAndW => write!(f, "N&W"),
            PubComId::PRR => write!(f, "PRR"),
            PubComId::PAndLE => write!(f, "P&LE"),
            PubComId::SRR => write!(f, "SRR"),
        }
    }
}

impl FromStr for PubComId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B&O" => Ok(PubComId::BAndO),
            "C&A" => Ok(PubComId::CAndA),
            "C&O" => Ok(PubComId::CAndO),
            "LV" => Ok(PubComId::LV),
            "N&W" => Ok(PubComId::NAndW),
            "PRR" => Ok(PubComId::PRR),
            "P&LE" => Ok(PubComId::PAndLE),
            "SRR" => Ok(PubComId::SRR),
            _ => Err(format!("{} can not be parsed as PubComId", s)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TerrainId {
    Plain,
    River,
    Mountain,
}

impl TerrainId {
    fn cost(&self) -> u32 {
        match self {
            TerrainId::Plain => 0,
            TerrainId::River => 40,
            TerrainId::Mountain => 80,
        }
    }
}

impl Default for TerrainId {
    fn default() -> Self {
        TerrainId::Plain
    }
}

impl FromStr for TerrainId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Plain" => Ok(TerrainId::Plain),
            "River" => Ok(TerrainId::River),
            "Mountain" => Ok(TerrainId::Mountain),
            _ => Err(format!("{} can not be parsed as TerrainId", s)),
        }
    }
}
