use crate::ids::PubComId;
use crate::INVALID_TOML;
use std::collections::HashMap;
use toml::Value;

static VALUES_MISSING: &str = "values is missing";
static VALUES_TYPEERROR: &str = "values is not of type Array of Arrays";
static VALUE_TYPEERROR: &str = "value is not of type Integer";

/// Represents the stock chart for a game
#[derive(Clone, Debug)]
pub struct StockChart {
    values: Vec<Vec<(i32)>>,
    tokens: HashMap<PubComId, (usize, usize, usize)>,
}

impl StockChart {
    /// Parses a `StockChart` from the TOML data in `s`
    pub fn from_toml(s: &str) -> Self {
        let toml = s.parse::<Value>().expect(INVALID_TOML);
        let mut values = Vec::new();
        let values_toml = toml.get("values").expect(VALUES_MISSING);
        for row_toml in values_toml.as_array().expect(VALUES_TYPEERROR) {
            let mut row = Vec::new();
            for value in row_toml.as_array().expect(VALUES_TYPEERROR) {
                let value = value.as_integer().expect(VALUE_TYPEERROR);
                row.push(value as i32);
            }
            values.push(row);
        }
        Self {
            values,
            tokens: HashMap::new(),
        }
    }
}
