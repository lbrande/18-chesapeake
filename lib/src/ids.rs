use std::fmt::{Display, Formatter, Result as FmtResult};
use std::slice::Iter;
use std::str::FromStr;

/// Represents the color of a tile
#[derive(Clone, Copy, Debug)]
pub enum ColorId {
    /// Yellow tile color
    Yellow,
    /// Green tile color
    Green,
    /// Brown tile color
    Brown,
    /// Gray tile color
    Gray,
    /// Red tile color
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

/// Represents the name of a public company
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PubComId {
    /// Baltimore and Ohio Railroad
    BAndO,
    /// Camden and Amboy Railroad
    CAndA,
    /// Chesapeake and Ohio Railroad
    CAndO,
    /// Lehigh Valley Railroad
    LV,
    /// Norfolk and Western Railway
    NAndW,
    /// Pennsylvania Railroad
    PRR,
    /// Pittsburgh & Lake Erie Railroad
    PAndLE,
    /// Strasburg Rail Road
    SRR,
}

impl PubComId {
    /// Returns the name corresponding to this id
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

    pub(crate) fn values() -> Iter<'static, Self> {
        [
            PubComId::BAndO,
            PubComId::CAndA,
            PubComId::CAndO,
            PubComId::LV,
            PubComId::NAndW,
            PubComId::PRR,
            PubComId::PAndLE,
            PubComId::SRR,
        ]
        .iter()
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

/// Represents the name of a private company
#[derive(Clone, Copy, Debug)]
pub enum PrivComId {
    /// Delaware and Raritan Canal
    DAndR,
    /// Columbia-Philadelphia Railroad
    CToP,
    /// Baltimore and Susquehanna Railroad
    BAndS,
    /// Chesapeake and Ohio Canal
    CAndO,
    /// Baltimore and Ohio Railroad
    BAndO,
    /// Cornelius Vanderbilt
    CV,
}

impl PrivComId {
    /// Returns the name corresponding to this id
    pub fn get_name(&self) -> &str {
        match self {
            PrivComId::DAndR => "Delaware and Raritan Canal",
            PrivComId::CToP => "Columbia-Philadelphia Railroad",
            PrivComId::BAndS => "Baltimore and Susquehanna Railroad",
            PrivComId::CAndO => "Chesapeake and Ohio Canal",
            PrivComId::BAndO => "Baltimore and Ohio Railroad",
            PrivComId::CV => "Cornelius Vanderbilt",
        }
    }
}

impl Display for PrivComId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PrivComId::DAndR => write!(f, "D&R"),
            PrivComId::CToP => write!(f, "C-P"),
            PrivComId::BAndS => write!(f, "B&S"),
            PrivComId::CAndO => write!(f, "C&O"),
            PrivComId::BAndO => write!(f, "B&O"),
            PrivComId::CV => write!(f, "CV"),
        }
    }
}

impl FromStr for PrivComId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D&R" => Ok(PrivComId::DAndR),
            "C-P" => Ok(PrivComId::CToP),
            "B&S" => Ok(PrivComId::BAndS),
            "C&O" => Ok(PrivComId::CAndO),
            "B&O" => Ok(PrivComId::BAndO),
            "CV" => Ok(PrivComId::CV),
            _ => Err(format!("{} can not be parsed as PrivComId", s)),
        }
    }
}

/// Represents the terrain of a hex
#[derive(Clone, Copy, Debug)]
pub enum TerrainId {
    /// Plain hex terrain
    Plain,
    /// River hex terrain
    River,
    /// Mountain hex terrain
    Mountain,
}

impl TerrainId {
    /// Returns the cost of laying track on this terrain
    pub fn cost(self) -> u32 {
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
