use crate::PubComId;
use std::collections::HashSet;
use toml::Value;

static VALUE_MISSING: &str = "value is missing";
static VALUE_TYPEERROR: &str = "value is not of type Integer";
static START_TYPEERROR: &str = "start is not of type String";
static SPOTS_MISSING: &str = "spots is missing";
static SPOTS_TYPEERROR: &str = "spots is not of type Integer";
static VALUES_MISSING: &str = "values is missing";
static VALUES_TYPEERROR: &str = "values is not of type Array";
static VALUES_LENERROR: &str = "values is not of length four";
static NAME_MISSING: &str = "name is missing";
static NAME_TYPEERROR: &str = "name is not of type String";

/// Represents a train stop
#[derive(Clone, Debug)]
pub enum Stop {
    /// Stop with a city
    City(City),
    /// Stop with an off-board location
    Location(Location),
}

/// Represents a city
#[derive(Clone, Debug)]
pub struct City {
    value: u32,
    stations: HashSet<PubComId>,
    spots: u32,
    name: Option<String>,
    start: Option<PubComId>,
}

impl City {
    pub(crate) fn from_toml(toml: &Value) -> Self {
        let value = toml
            .get("value")
            .expect(VALUE_MISSING)
            .as_integer()
            .expect(VALUE_TYPEERROR) as u32;
        let spots = toml
            .get("spots")
            .expect(SPOTS_MISSING)
            .as_integer()
            .expect(SPOTS_TYPEERROR) as u32;
        let name = toml
            .get("name")
            .and_then(|n| Some(n.as_str().expect(NAME_TYPEERROR).to_string()));
        let start = toml
            .get("start")
            .and_then(|t| Some(t.as_str().expect(START_TYPEERROR)))
            .and_then(|t| Some(t.parse::<PubComId>().unwrap()));
        Self {
            value,
            stations: HashSet::new(),
            spots,
            name,
            start,
        }
    }
}

/// Represents an off-board location
#[derive(Clone, Debug)]
pub struct Location {
    values: (u32, u32, u32, u32),
    stations: HashSet<PubComId>,
    name: String,
    start: Option<PubComId>,
}

impl Location {
    pub(crate) fn from_toml(toml: &Value) -> Self {
        let values = toml
            .get("values")
            .expect(VALUES_MISSING)
            .as_array()
            .expect(VALUES_TYPEERROR);
        if values.len() != 4 {
            panic!(VALUES_LENERROR);
        }
        let values = (
            values[0].as_integer().expect(VALUE_TYPEERROR) as u32,
            values[1].as_integer().expect(VALUE_TYPEERROR) as u32,
            values[2].as_integer().expect(VALUE_TYPEERROR) as u32,
            values[3].as_integer().expect(VALUE_TYPEERROR) as u32,
        );
        let name = toml
            .get("name")
            .expect(NAME_MISSING)
            .as_str()
            .expect(NAME_TYPEERROR);
        let start = toml
            .get("start")
            .and_then(|t| Some(t.as_str().expect(START_TYPEERROR)))
            .and_then(|t| Some(t.parse::<PubComId>().unwrap()));
        Self {
            values,
            stations: HashSet::new(),
            name: name.to_string(),
            start,
        }
    }
}
