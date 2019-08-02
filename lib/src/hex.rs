use crate::ids::TerrainId;
use crate::rail::City;
use crate::tile::Tile;
use toml::Value;

static TERRAIN_TYPEERROR: &str = "terrain is not of type String";
static CITIES_TYPEERROR: &str = "cities is not of type Array";

#[derive(Clone, Debug)]
pub struct Hex {
    terrain: TerrainId,
    content: Option<Content>,
}

impl Hex {
    pub fn from_toml(toml: &Value) -> Self {
        let terrain = toml.get("terrain");
        let terrain = terrain.and_then(|t| Some(t.as_str().expect(TERRAIN_TYPEERROR)));
        let terrain = terrain.and_then(|t| Some(t.parse::<TerrainId>().unwrap()));
        let terrain = terrain.unwrap_or_default();
        if toml.get("rails").is_some() && toml.get("color").is_some() {
            let mut toml = toml.clone();
            let id = Value::Integer(0);
            toml.as_table_mut().unwrap().insert("id".to_string(), id);
            let tile = Tile::from_toml(&toml);
            Self {
                terrain,
                content: Some(Content::Tile(tile)),
            }
        } else if let Some(value) = toml.get("cities") {
            let mut cities = Vec::new();
            for value in value.as_array().expect(CITIES_TYPEERROR) {
                cities.push(City::from_toml(value));
            }
            Self {
                terrain,
                content: Some(Content::Cities(cities)),
            }
        } else {
            Self {
                terrain,
                content: None,
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Content {
    Tile(Tile),
    Cities(Vec<City>),
}
