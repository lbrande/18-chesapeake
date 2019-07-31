use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::*;
use std::str::FromStr;
use std::u32;
use toml::Value;

static INVALID_TOML: &str = "TOML is invalid";
static TILES_MISSING: &str = "tiles is missing";
static TILES_TYPEERROR: &str = "tiles is not of type Array";
static COUNT_MISSING: &str = "count is missing";
static COUNT_TYPEERROR: &str = "count is not of type Integer";
static ID_MISSING: &str = "id is missing";
static ID_TYPEERROR: &str = "id is not of type Integer";
static RAILS_MISSING: &str = "rails is missing";
static RAILS_TYPEERROR: &str = "rails is not of type Array";
static COLOR_MISSING: &str = "color is missing";
static COLOR_TYPEERROR: &str = "color is not of type String";
static UPGRADES_MISSING: &str = "upgrades is missing";
static UPGRADES_TYPEERROR: &str = "upgrades is not of type Array";
static UPGRADE_TYPEERROR: &str = "upgrade is not of type Integer";
static EDGES_MISSING: &str = "edges is missing";
static EDGES_TYPEERROR: &str = "edges is not of type Array";
static EDGE_TYPEERROR: &str = "edge is not of type Integer";
static VALUE_MISSING: &str = "value is missing";
static VALUE_TYPEERROR: &str = "value is not of type Integer";
static STATIONS_TYPEERROR: &str = "stations is not of type Array";
static STATION_TYPEERROR: &str = "station is not of type String";
static SPOTS_MISSING: &str = "spots is missing";
static SPOTS_TYPEERROR: &str = "spots is not of type Integer";
static VALUES_MISSING: &str = "values is missing";
static VALUES_TYPEERROR: &str = "values is not of type Array";
static VALUES_LENERROR: &str = "values is not of length four";

#[derive(Debug)]
pub struct TileSet {
    tiles: HashSet<(Tile, i32)>,
}

impl FromStr for TileSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toml = s.parse::<Value>().or(Err(INVALID_TOML))?;
        let mut tiles = HashSet::new();
        let tiles_toml = toml.get("tiles").ok_or(TILES_MISSING)?;
        for value in tiles_toml.as_array().ok_or(TILES_TYPEERROR)? {
            let tile = Tile::from_toml(value);
            let count = value.get("count").ok_or(COUNT_MISSING)?;
            let count = count.as_integer().ok_or(COUNT_TYPEERROR)?;
            tiles.insert((tile, count as i32));
        }
        Ok(Self { tiles })
    }
}

#[derive(Debug)]
pub struct Tile {
    id: i32,
    rails: Vec<Rail>,
    color: ColorId,
    upgrades: Vec<i32>,
}

impl Tile {
    pub fn from_toml(toml: &Value) -> Self {
        let id = toml.get("id").expect(ID_MISSING);
        let id = id.as_integer().expect(ID_TYPEERROR) as i32;
        let mut rails = Vec::new();
        let rails_toml = toml.get("rails").expect(RAILS_MISSING);
        for value in rails_toml.as_array().expect(RAILS_TYPEERROR) {
            rails.push(Rail::from_toml(value));
        }
        let color = toml.get("color").expect(COLOR_MISSING);
        let color = color.as_str().expect(COLOR_TYPEERROR);
        let mut upgrades = Vec::new();
        let upgrades_toml = toml.get("upgrades").expect(UPGRADES_MISSING);
        for value in upgrades_toml.as_array().expect(UPGRADES_TYPEERROR) {
            upgrades.push(value.as_integer().expect(UPGRADE_TYPEERROR) as i32);
        }
        Self {
            id,
            rails,
            color: color.parse::<ColorId>().unwrap(),
            upgrades,
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Debug)]
pub struct Rail {
    edges: Vec<u32>,
    stop: Option<Stop>,
}

impl Rail {
    pub fn from_toml(toml: &Value) -> Self {
        let mut edges = Vec::new();
        let edges_toml = toml.get("edges").expect(EDGES_MISSING);
        for value in edges_toml.as_array().expect(EDGES_TYPEERROR) {
            edges.push(value.as_integer().expect(EDGE_TYPEERROR) as u32);
        }
        if let Some(value) = toml.get("city") {
            let city = City::from_toml(value);
            Self {
                edges,
                stop: Some(Stop::City(city)),
            }
        } else if let Some(value) = toml.get("location") {
            let location = Location::from_toml(value);
            Self {
                edges,
                stop: Some(Stop::Location(location)),
            }
        } else {
            Self { edges, stop: None }
        }
    }
}

#[derive(Debug)]
pub enum Stop {
    City(City),
    Location(Location),
}

#[derive(Debug)]
pub struct City {
    value: u32,
    stations: HashSet<PubComId>,
    spots: u32,
}

impl City {
    pub fn from_toml(toml: &Value) -> Self {
        let value = toml.get("value").expect(VALUE_MISSING);
        let value = value.as_integer().expect(VALUE_TYPEERROR) as u32;
        let mut stations = HashSet::new();
        if let Some(stations_toml) = toml.get("stations") {
            for value in stations_toml.as_array().expect(STATIONS_TYPEERROR) {
                let station = value.as_str().expect(STATION_TYPEERROR);
                stations.insert(station.parse::<PubComId>().unwrap());
            }
        }
        let spots = toml.get("spots").expect(SPOTS_MISSING);
        let spots = spots.as_integer().expect(SPOTS_TYPEERROR) as u32;
        Self {
            value,
            stations,
            spots,
        }
    }
}

#[derive(Debug)]
pub struct Location {
    values: (u32, u32, u32, u32),
    stations: HashSet<PubComId>,
}

impl Location {
    pub fn from_toml(toml: &Value) -> Self {
        let values = toml.get("values").expect(VALUES_MISSING);
        let values = values.as_array().expect(VALUES_TYPEERROR);
        if values.len() != 4 {
            panic!(VALUES_LENERROR);
        }
        let values = (
            values[0].as_integer().expect(VALUE_TYPEERROR) as u32,
            values[1].as_integer().expect(VALUE_TYPEERROR) as u32,
            values[2].as_integer().expect(VALUE_TYPEERROR) as u32,
            values[3].as_integer().expect(VALUE_TYPEERROR) as u32,
        );
        let mut stations = HashSet::new();
        if let Some(stations_toml) = toml.get("stations") {
            for value in stations_toml.as_array().expect(STATIONS_TYPEERROR) {
                let station = value.as_str().expect(STATION_TYPEERROR);
                stations.insert(station.parse::<PubComId>().unwrap());
            }
        }
        Self { values, stations }
    }
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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
