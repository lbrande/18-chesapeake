use crate::ids::TerrainId;
use crate::tile::Tile;
use toml::Value;

#[derive(Debug)]
pub struct Hex {
    terrain: TerrainId,
    tile: Option<Tile>,
}

impl Hex {
    pub fn from_toml(toml: &Value) -> Self {
        Self {
            terrain: TerrainId::Plain,
            tile: None,
        }
    }
}
