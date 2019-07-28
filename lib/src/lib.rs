#![allow(dead_code)]

use std::collections::*;
use std::u32;
use toml::Value;

#[derive(Default)]
pub struct TileSet {
    tiles: HashMap<i32, (Tile, i32)>,
}

impl TileSet {
    pub fn from_string(string: &str) -> Self {
        let mut tiles = HashMap::new();
        for (key, value) in string.parse::<Value>().unwrap().as_table().unwrap() {
            let tile = Tile::from_toml(value);
            let count = value.get("count").unwrap().as_integer().unwrap();
            tiles.insert(key.parse::<i32>().unwrap(), (tile, count as i32));
        }
        Self { tiles }
    }
}

pub struct Tile {
    rails: Vec<Rail>,
    color: ColorId,
}

impl Tile {
    pub fn from_toml(toml: &Value) -> Self {
        let mut rails = Vec::new();
        for value in toml.get("rails").unwrap().as_array().unwrap() {
            rails.push(Rail::from_toml(value));
        }
        let color = toml.get("color").unwrap().as_str().unwrap();
        Self {
            rails,
            color: ColorId::from_string(color),
        }
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
            _ => panic!("{} is not a valid ColorId", string),
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
