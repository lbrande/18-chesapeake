use crate::hex::Hex;
use crate::tile::Tile;
use std::collections::HashSet;
use std::str::FromStr;
use toml::Value;

pub mod hex;
pub mod ids;
pub mod rail;
pub mod tile;

static INVALID_TOML: &str = "TOML is invalid";
static TILES_MISSING: &str = "tiles is missing";
static TILES_TYPEERROR: &str = "tiles is not of type Array";
static COUNT_MISSING: &str = "count is missing";
static COUNT_TYPEERROR: &str = "count is not of type Integer";
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

#[derive(Debug)]
pub struct Map {
    hexes: Vec<Vec<Option<Hex>>>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toml = s.parse::<Value>().or(Err(INVALID_TOML))?;
        let width = toml.get("width").ok_or(WIDTH_MISSING)?;
        let width = width.as_integer().ok_or(WIDTH_TYPEERROR)?;
        let height = toml.get("height").ok_or(HEIGHT_MISSING)?;
        let height = height.as_integer().ok_or(HEIGHT_TYPEERROR)?;
        let mut hexes = vec![vec![None; height as usize]; width as usize];
        let hexes_toml = toml.get("hexes").ok_or(HEXES_MISSING)?;
        for value in hexes_toml.as_array().ok_or(HEXES_TYPEERROR)? {
            let hex = Hex::from_toml(value);
            let x = value.get("x").ok_or(X_MISSING)?;
            let x = x.as_integer().ok_or(X_TYPEERROR)?;
            let y = value.get("y").ok_or(Y_MISSING)?;
            let y = y.as_integer().ok_or(Y_TYPEERROR)?;
            hexes[x as usize][y as usize] = Some(hex);
        }
        Ok(Self { hexes })
    }
}
