use super::Rail;
use crate::ColorId;
use toml::Value;

static RAILS_MISSING: &str = "rails is missing";
static RAILS_TYPEERROR: &str = "rails is not of type Array";
static COLOR_MISSING: &str = "color is missing";
static COLOR_TYPEERROR: &str = "color is not of type String";
static UPGRADES_TYPEERROR: &str = "upgrades is not of type Array";
static UPGRADE_TYPEERROR: &str = "upgrade is not of type Integer";

/// Represents a tile
#[derive(Clone, Debug)]
pub struct Tile {
    rails: Vec<Rail>,
    color: ColorId,
    upgrades: Vec<i32>,
}

impl Tile {
    pub(crate) fn from_toml(toml: &Value) -> Self {
        let mut rails = Vec::new();
        let rails_toml = toml.get("rails").expect(RAILS_MISSING);
        for value in rails_toml.as_array().expect(RAILS_TYPEERROR) {
            rails.push(Rail::from_toml(value));
        }
        let color = toml
            .get("color")
            .expect(COLOR_MISSING)
            .as_str()
            .expect(COLOR_TYPEERROR);
        let mut upgrades = Vec::new();
        if let Some(value) = toml.get("upgrades") {
            for value in value.as_array().expect(UPGRADES_TYPEERROR) {
                let upgrade = value.as_integer().expect(UPGRADE_TYPEERROR);
                upgrades.push(upgrade as i32);
            }
        }
        Self {
            rails,
            color: color.parse::<ColorId>().unwrap(),
            upgrades,
        }
    }
}
