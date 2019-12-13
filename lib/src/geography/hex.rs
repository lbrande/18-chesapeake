use super::{City, Tile};
use crate::{PrivComId, PubComId, TerrainId};
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
            .map(|t| t.as_str().expect(TERRAIN_TYPEERROR))
            .map(|t| t.parse::<TerrainId>().unwrap())
            .unwrap_or_default();
        let private = toml
            .get("private")
            .map(|p| p.as_str().expect(PRIVATE_TYPEERROR))
            .map(|p| p.parse::<PrivComId>().unwrap());
        let tile = toml
            .get("rails")
            .and(toml.get("color"))
            .map(|_| Content::Tile(Tile::from_toml(&toml)));
        let cities = toml.get("cities").map(|cs| {
            Content::Cities(
                cs.as_array()
                    .expect(CITIES_TYPEERROR)
                    .iter()
                    .map(|c| City::from_toml(c))
                    .collect(),
            )
        });
        Self {
            terrain,
            content: tile.or(cities),
            private,
        }
    }

    pub(crate) fn place_station(&mut self, pub_com: PubComId, edge: Option<u32>) {
        if let Some(content) = &mut self.content {
            match content {
                Content::Tile(tile) => {
                    if let Some(edge) = edge {
                        for rail in tile.rails_mut() {
                            if rail.edges().contains(&edge) {
                                rail.place_station(pub_com);
                                break;
                            }
                        }
                    } else {
                        tile.rails_mut()[0].place_station(pub_com);
                    }
                }
                Content::Cities(cities) => {
                    cities[0].place_station(pub_com);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Content {
    Tile(Tile),
    Cities(Vec<City>),
}
