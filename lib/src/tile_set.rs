use crate::tile::Tile;
use crate::INVALID_TOML;
use std::collections::HashSet;
use toml::Value;

static TILES_MISSING: &str = "tiles is missing";
static TILES_TYPEERROR: &str = "tiles is not of type Array";
static COUNT_MISSING: &str = "count is missing";
static COUNT_TYPEERROR: &str = "count is not of type Integer";

/// Represents the tiles and their respective counts avaliable in a game
#[derive(Clone, Debug)]
pub struct TileSet {
    tiles: HashSet<(Tile, i32)>,
}

impl TileSet {
    /// Parses a `TileSet` from the TOML data in `s`
    pub fn from_toml(s: &str) -> Self {
        let toml = s.parse::<Value>().expect(INVALID_TOML);
        let mut tiles = HashSet::new();
        let tiles_toml = toml.get("tiles").expect(TILES_MISSING);
        for value in tiles_toml.as_array().expect(TILES_TYPEERROR) {
            let tile = Tile::from_toml(value);
            let count = value
                .get("count")
                .expect(COUNT_MISSING)
                .as_integer()
                .expect(COUNT_TYPEERROR);
            tiles.insert((tile, count as i32));
        }
        Self { tiles }
    }
}
