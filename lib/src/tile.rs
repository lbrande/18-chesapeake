use crate::ids::ColorId;
use crate::rail::Rail;
use std::hash::*;
use toml::Value;

static ID_MISSING: &str = "id is missing";
static ID_TYPEERROR: &str = "id is not of type Integer";
static RAILS_MISSING: &str = "rails is missing";
static RAILS_TYPEERROR: &str = "rails is not of type Array";
static COLOR_MISSING: &str = "color is missing";
static COLOR_TYPEERROR: &str = "color is not of type String";
static UPGRADES_TYPEERROR: &str = "upgrades is not of type Array";
static UPGRADE_TYPEERROR: &str = "upgrade is not of type Integer";

#[derive(Clone, Debug)]
pub struct Tile {
    id: i32,
    rails: Vec<Rail>,
    color: ColorId,
    upgrades: Vec<i32>,
}

impl Tile {
    pub fn from_toml(toml: &Value) -> Self {
        let id = toml
            .get("id")
            .expect(ID_MISSING)
            .as_integer()
            .expect(ID_TYPEERROR) as i32;
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
            id,
            rails,
            color: color.parse::<ColorId>().unwrap(),
            upgrades,
        }
    }

    pub fn from_toml_no_id(toml: &Value) -> Self {
        let mut toml = toml.clone();
        toml.as_table_mut()
            .unwrap()
            .insert("id".to_string(), Value::Integer(0));
        Tile::from_toml(&toml)
    }
}

impl PartialEq for Tile {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
