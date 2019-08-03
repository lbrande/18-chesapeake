use crate::INVALID_TOML;
use toml::Value;
use std::str::FromStr;
use crate::tile::Tile;
use std::collections::HashSet;

static TILES_MISSING: &str = "tiles is missing";
static TILES_TYPEERROR: &str = "tiles is not of type Array";
static COUNT_MISSING: &str = "count is missing";
static COUNT_TYPEERROR: &str = "count is not of type Integer";

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