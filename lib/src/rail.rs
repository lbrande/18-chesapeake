use crate::ids::PubComId;
use std::collections::HashSet;
use toml::Value;

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
static NAME_MISSING: &str = "name is missing";
static NAME_TYPEERROR: &str = "name is not of type String";

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
enum Stop {
    City(City),
    Location(Location),
}

#[derive(Clone, Debug)]
struct City {
    value: u32,
    stations: HashSet<PubComId>,
    spots: u32,
    name: Option<String>,
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
        if let Some(name) = toml.get("name") {
        let name = name.as_str().expect(NAME_TYPEERROR);
        Self {
            value,
            stations,
            spots,
            name: Some(name.to_string())
        }
        } else {
            Self {
            value,
            stations,
            spots,
            name: None
        }
        }
    }
}

#[derive(Clone, Debug)]
struct Location {
    values: (u32, u32, u32, u32),
    stations: HashSet<PubComId>,
    name: String,
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
        let name = toml.get("name").expect(NAME_MISSING);
        let name = name.as_str().expect(NAME_TYPEERROR);
        Self {
            values,
            stations,
            name: name.to_string(),
        }
    }
}
