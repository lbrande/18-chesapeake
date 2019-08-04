use crate::ids::PubComId;
use crate::INVALID_TOML;
use std::collections::HashMap;
use toml::Value;

static VALUES_MISSING: &str = "values is missing";
static VALUES_TYPEERROR: &str = "values is not of type Array";
static VALUE_TYPEERROR: &str = "value is not of type Integer";

/// Represents the par track for a game
#[derive(Clone, Debug)]
pub struct ParTrack {
    values: Vec<u32>,
    tokens: HashMap<PubComId, usize>,
}

impl ParTrack {
    /// Parses a `ParTrack` from the TOML data in `s`
    pub fn from_toml(s: &str) -> Self {
        let toml = s.parse::<Value>().expect(INVALID_TOML);
        let mut values = Vec::new();
        let values_toml = toml.get("values").expect(VALUES_MISSING);
        for value in values_toml.as_array().expect(VALUES_TYPEERROR) {
            let value = value.as_integer().expect(VALUE_TYPEERROR);
            values.push(value as u32);
        }
        Self {
            values,
            tokens: HashMap::new(),
        }
    }
}
