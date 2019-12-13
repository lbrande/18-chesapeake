use super::{City, Stop, Tile};
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

    pub(crate) fn place_station(&mut self, pub_com: PubComId, from_edge: u32) {
        if let Some(content) = &mut self.content {
            match content {
                Content::Tile(tile) => {
                    for rail in tile.rails_mut() {
                        if rail.edges().contains(&from_edge) {
                            rail.place_station(pub_com);
                            break;
                        }
                    }
                }
                Content::Cities(cities) => {
                    cities[0].place_station(pub_com);
                }
            }
        }
    }

    /// Returns the public company that has its home in this `Hex` and its `from_edge`, if any
    pub fn home(&self) -> Option<(PubComId, u32)> {
        if let Some(content) = &self.content {
            match content {
                Content::Tile(tile) => {
                    for rail in tile.rails() {
                        if let Some(stop) = rail.stop() {
                            match stop {
                                Stop::City(city) => {
                                    return city.home().map(|h| (h, rail.edges()[0]))
                                }
                                Stop::Location(location) => {
                                    return location.home().map(|h| (h, rail.edges()[0]))
                                }
                            }
                        }
                    }
                }
                Content::Cities(cities) => {
                    for city in cities {
                        if let Some(home) = city.home() {
                            return Some((home, 0));
                        }
                    }
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
enum Content {
    Tile(Tile),
    Cities(Vec<City>),
}
