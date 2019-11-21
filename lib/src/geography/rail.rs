use super::{City, Location, Stop};
use toml::Value;

static EDGES_MISSING: &str = "edges is missing";
static EDGES_TYPEERROR: &str = "edges is not of type Array";
static EDGE_TYPEERROR: &str = "edge is not of type Integer";

#[derive(Clone, Debug)]
///Represents a rail
pub struct Rail {
    edges: Vec<u32>,
    stop: Option<Stop>,
}

impl Rail {
    pub(crate) fn from_toml(toml: &Value) -> Self {
        let mut edges = Vec::new();
        let edges_toml = toml.get("edges").expect(EDGES_MISSING);
        for value in edges_toml.as_array().expect(EDGES_TYPEERROR) {
            let edge = value.as_integer().expect(EDGE_TYPEERROR);
            edges.push(edge as u32);
        }
        let city = toml
            .get("city")
            .and_then(|c| Some(Stop::City(City::from_toml(c))));
        let location = toml
            .get("location")
            .and_then(|l| Some(Stop::Location(Location::from_toml(l))));
        Self {
            edges,
            stop: city.or(location),
        }
    }

    pub(crate) fn stop(&self) -> &Option<Stop> {
        &self.stop
    }
}
