use super::Hex;
use crate::geography::TrackLayMap;
use crate::{PubComId, INVALID_TOML};
use std::collections::HashMap;
use toml::Value;

static WIDTH_MISSING: &str = "width is missing";
static WIDTH_TYPEERROR: &str = "width is not of type Integer";
static HEIGHT_MISSING: &str = "height is missing";
static HEIGHT_TYPEERROR: &str = "height is not of type Integer";
static HEXES_MISSING: &str = "hexes is missing";
static HEXES_TYPEERROR: &str = "hexes is not of type Array";
static X_MISSING: &str = "x is missing";
static X_TYPEERROR: &str = "x is not of type Integer";
static Y_MISSING: &str = "y is missing";
static Y_TYPEERROR: &str = "y is not of type Integer";
static HOMES_MISSING: &str = "homes is missing";
static HOMES_TYPEERROR: &str = "homes is not of type Array";
static ID_MISSING: &str = "id is missing";
static ID_TYPEERROR: &str = "id is not of type String";
static EDGE_TYPEERROR: &str = "edge is not of type Integer";

/// Represents the map that a game is played on
#[derive(Clone, Debug)]
pub struct Map {
    width: usize,
    height: usize,
    hexes: Vec<Vec<Option<Hex>>>,
    homes: HashMap<PubComId, (usize, usize, Option<u32>)>,
    stations: HashMap<PubComId, Vec<(usize, usize, Option<u32>)>>,
}

impl Map {
    pub(crate) fn from_toml(s: &str) -> Self {
        let toml = s.parse::<Value>().expect(INVALID_TOML);
        let width = toml
            .get("width")
            .expect(WIDTH_MISSING)
            .as_integer()
            .expect(WIDTH_TYPEERROR) as usize;
        let height = toml
            .get("height")
            .expect(HEIGHT_MISSING)
            .as_integer()
            .expect(HEIGHT_TYPEERROR) as usize;
        let mut hexes = vec![vec![None; height]; width];
        let hexes_toml = toml.get("hexes").expect(HEXES_MISSING);
        for value in hexes_toml.as_array().expect(HEXES_TYPEERROR) {
            let x = value
                .get("x")
                .expect(X_MISSING)
                .as_integer()
                .expect(X_TYPEERROR) as usize;
            let y = value
                .get("y")
                .expect(Y_MISSING)
                .as_integer()
                .expect(Y_TYPEERROR) as usize;
            if hexes[x][y].is_some() {
                panic!("hex at x={}, y={} is not empty", x, y);
            }
            let hex = Hex::from_toml(value);
            hexes[x][y] = Some(hex);
        }
        let mut homes = HashMap::new();
        let homes_toml = toml.get("homes").expect(HOMES_MISSING);
        for value in homes_toml.as_array().expect(HOMES_TYPEERROR) {
            let id = value
                .get("id")
                .expect(ID_MISSING)
                .as_str()
                .expect(ID_TYPEERROR)
                .parse::<PubComId>()
                .unwrap();
            let x = value
                .get("x")
                .expect(X_MISSING)
                .as_integer()
                .expect(X_TYPEERROR) as usize;
            let y = value
                .get("y")
                .expect(Y_MISSING)
                .as_integer()
                .expect(Y_TYPEERROR) as usize;
            let edge = value
                .get("edge")
                .map(|e| e.as_integer().expect(EDGE_TYPEERROR))
                .map(|e| e as u32);
            homes.insert(id, (x, y, edge));
        }
        let mut stations = HashMap::new();
        for id in PubComId::values() {
            stations.insert(id, Vec::new());
        }
        Self {
            width,
            height,
            hexes,
            homes,
            stations,
        }
    }

    pub(crate) fn place_home_station(&mut self, pub_com: PubComId) {
        if let Some(&(x, y, edge)) = self.homes.get(&pub_com) {
            if let Some(hex) = &mut self.hexes[x][y] {
                hex.place_station(pub_com, edge);
            }
        }
        unreachable!()
    }

    /// Returns the `TrackLayMap` for `pub_com` on this `Map`.
    pub fn track_lay_map(&self, pub_com: PubComId) -> TrackLayMap {
        let map = TrackLayMap::new();
        if let Some(stations) = self.stations.get(&pub_com) {
            for &(x, y, edge) in stations {}
        } else {
            unreachable!();
        }
        map
    }
}
