use crate::ids::TerrainId;
use crate::tile::Tile;
use toml::Value;

static TERRAIN_MISSING: &str = "terrain is missing";
static TERRAIN_TYPEERROR: &str = "terrain is not of type String";

#[derive(Clone, Debug, Default)]
pub struct Hex {
    terrain: TerrainId,
    tile: Option<Tile>,
}

impl Hex {
    pub fn from_toml(toml: &Value) -> Self {
        let terrain = toml.get("terrain").expect(TERRAIN_MISSING);
        let terrain = terrain.as_str().expect(TERRAIN_TYPEERROR);
        let tile = Tile::from_toml(toml);
        Self {
            terrain: terrain.parse::<TerrainId>().unwrap(),
            tile: Some(tile),
        }
    }
}
