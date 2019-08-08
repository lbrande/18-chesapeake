use super::Tile;
use crate::INVALID_TOML;
use std::collections::HashMap;
use toml::Value;

static TILES_MISSING: &str = "tiles is missing";
static TILES_TYPEERROR: &str = "tiles is not of type Array";
static ID_MISSING: &str = "id is missing";
static ID_TYPEERROR: &str = "id is not of type Integer";
static COUNT_MISSING: &str = "count is missing";
static COUNT_TYPEERROR: &str = "count is not of type Integer";

/// Represents the tiles and their respective counts avaliable in a game
#[derive(Clone, Debug)]
pub struct TileSet {
    tiles: HashMap<i32, (Tile, i32)>,
}

impl TileSet {
    pub(crate) fn from_toml(s: &str) -> Self {
        let toml = s.parse::<Value>().expect(INVALID_TOML);
        let mut tiles = HashMap::new();
        let tiles_toml = toml.get("tiles").expect(TILES_MISSING);
        for value in tiles_toml.as_array().expect(TILES_TYPEERROR) {
            let id = value
                .get("id")
                .expect(ID_MISSING)
                .as_integer()
                .expect(ID_TYPEERROR) as i32;
            if tiles.contains_key(&id) {
                panic!("tile with id={} already exists", id);
            }
            let count = value
                .get("count")
                .expect(COUNT_MISSING)
                .as_integer()
                .expect(COUNT_TYPEERROR);
            let tile = Tile::from_toml(value);
            tiles.insert(id, (tile, count as i32));
        }
        Self { tiles }
    }
}
