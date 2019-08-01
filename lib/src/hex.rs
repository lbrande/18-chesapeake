use crate::ids::TerrainId;
use crate::tile::Tile;
use toml::Value;

static TERRAIN_TYPEERROR: &str = "terrain is not of type String";

#[derive(Clone, Debug, Default)]
pub struct Hex {
    terrain: TerrainId,
    tile: Option<Tile>,
}

impl Hex {
    pub fn from_toml(toml: &Value) -> Self {
        let terrain = toml.get("terrain");
        let terrain = terrain.and_then(|t| Some(t.as_str().expect(TERRAIN_TYPEERROR)));
        let terrain = terrain.and_then(|t| Some(t.parse::<TerrainId>().unwrap()));
        let terrain = terrain.unwrap_or_default();
        if toml.get("rails").is_some() && toml.get("color").is_some() {
            let tile = Tile::from_toml(toml);
            Self {
                terrain,
                tile: Some(tile),
            }
        } else {
            Self {
                terrain,
                tile: None,
            }
        }
    }
}
