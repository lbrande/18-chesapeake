#![allow(dead_code)]

use std::collections::HashSet;
use std::hash::*;
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
static UPGRADE_TYPEERROR: &str = "upgrades is not of type Integer";

pub struct TileSet {
    tiles: HashSet<(Tile, i32)>,
}

impl TileSet {
    pub fn from_string(string: &str) -> Self {
        let toml = string.parse::<Value>().expect(INVALID_TOML);
        let mut tiles = HashSet::new();
        let tiles_toml = toml.get("tiles").expect(TILES_MISSING);
        for value in tiles_toml.as_array().expect(TILES_TYPEERROR) {
            let tile = Tile::from_toml(value);
            let count = value.get("count").expect(COUNT_MISSING);
            let count = count.as_integer().expect(COUNT_TYPEERROR);
            tiles.insert((tile, count as i32));
        }
        Self { tiles }
    }
}

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
            color: ColorId::from_string(color),
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

pub struct Rail {
    edges: Vec<u32>,
    stop: Option<Stop>,
}

impl Rail {
    pub fn from_toml(toml: &Value) -> Self {
        Self {
            edges: Vec::new(),
            stop: None,
        }
    }
}

pub enum Stop {
    City(City),
    Location(Location),
}

pub struct City {
    value: u32,
    stations: HashSet<PubComId>,
    spots: u32,
}

impl City {
    pub fn new(value: u32, spots: u32) -> Self {
        Self {
            value,
            stations: HashSet::new(),
            spots,
        }
    }

    pub fn with_station(value: u32, spots: u32, station: PubComId) -> Self {
        Self {
            value,
            stations: vec![station].iter().copied().collect(),
            spots,
        }
    }
}

pub struct Location {
    values: (u32, u32, u32, u32),
    stations: HashSet<PubComId>,
}

impl Location {
    pub fn new(values: (u32, u32, u32, u32)) -> Self {
        Self {
            values,
            stations: HashSet::new(),
        }
    }

    pub fn with_station(values: (u32, u32, u32, u32), station: PubComId) -> Self {
        Self {
            values,
            stations: vec![station].iter().copied().collect(),
        }
    }
}

pub enum ColorId {
    Yellow,
    Green,
    Brown,
    Gray,
    Red,
}

impl ColorId {
    pub fn from_string(string: &str) -> Self {
        match string {
            "Yellow" => ColorId::Yellow,
            "Green" => ColorId::Green,
            "Brown" => ColorId::Brown,
            "Gray" => ColorId::Gray,
            "Red" => ColorId::Red,
            _ => panic!("{} can not be parsed as ColorId", string),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
            PubComId::BAndO => "B&O",
            PubComId::CAndA => "C&A",
            PubComId::CAndO => "C&O",
            PubComId::LV => "LV",
            PubComId::NAndW => "N&W",
            PubComId::PRR => "PRR",
            PubComId::PAndLE => "P&LE",
            PubComId::SRR => "SRR",
        }
    }

    pub fn get_full_name(&self) -> &str {
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
