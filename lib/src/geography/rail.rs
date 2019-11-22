use crate::PubComId;
use super::{City, Location, Stop};
use toml::Value;

static EDGES_MISSING: &str = "edges is missing";
static EDGES_TYPEERROR: &str = "edges is not of type Array";
static EDGE_TYPEERROR: &str = "edge is not of type Integer";
static ACTION_FORBIDDEN: &str = "action is forbidden";

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

    pub(crate) fn place_station(&mut self, pub_com: PubComId) {
        if let Some(stop) = &mut self.stop {
            match stop {
                Stop::City(city) => {
                    city.place_station(pub_com);
                },
                Stop::Location(location) => {
                    location.place_station(pub_com);
                }
            }
        } else {
            panic!(ACTION_FORBIDDEN);
        }
    }

    /// Returns the edges of this `Rail`
    pub fn edges(&self) -> &[u32] {
        &self.edges
    }

    /// Returns the stop of this rail, if any
    pub fn stop(&self) -> Option<&Stop> {
        self.stop.as_ref()
    }
}
