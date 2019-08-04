use crate::ids::PrivComId;
use crate::ids::TerrainId;
use crate::stops::City;
use crate::tile::Tile;
use toml::Value;

static TERRAIN_TYPEERROR: &str = "terrain is not of type String";
static PRIVATE_TYPEERROR: &str = "private is not of type String";
static CITIES_TYPEERROR: &str = "cities is not of type Array";

#[derive(Clone, Debug)]
/// Represents a hex
pub struct Hex {
    terrain: TerrainId,
    content: Option<Content>,
    private: Option<PrivComId>,
}

impl Hex {
    pub(crate) fn from_toml(toml: &Value) -> Self {
        let terrain = toml
            .get("terrain")
            .and_then(|t| Some(t.as_str().expect(TERRAIN_TYPEERROR)))
            .and_then(|t| Some(t.parse::<TerrainId>().unwrap()))
            .unwrap_or_default();
        let private = toml
            .get("private")
            .and_then(|p| Some(p.as_str().expect(PRIVATE_TYPEERROR)))
            .and_then(|p| Some(p.parse::<PrivComId>().unwrap()));
        let tile = toml
            .get("rails")
            .and(toml.get("color"))
            .and_then(|_| Some(Content::Tile(Tile::from_toml_no_id(&toml))));
        let cities = toml.get("cities").and_then(|cs| {
            Some(Content::Cities(
                cs.as_array()
                    .expect(CITIES_TYPEERROR)
                    .iter()
                    .map(|c| City::from_toml(c))
                    .collect(),
            ))
        });
        Self {
            terrain,
            content: tile.or(cities),
            private,
        }
    }
}

#[derive(Clone, Debug)]
enum Content {
    Tile(Tile),
    Cities(Vec<City>),
}
