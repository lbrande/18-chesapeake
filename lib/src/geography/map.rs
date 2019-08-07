use super::Hex;
use crate::INVALID_TOML;
use toml::Value;

static WIDTH_MISSING: &str = "width is missing";
static WIDTH_TYPEERROR: &str = "width is not of type Integer";
static HEIGHT_MISSING: &str = "height is missing";
static HEIGHT_TYPEERROR: &str = "height is not of type Integer";
static HEXES_MISSING: &str = "hexes is missing";
static HEXES_TYPEERROR: &str = "hexes is not of type Array";
static X_MISSING: &str = "x is missing";
static X_TYPEERROR: &str = "x is not of type Integer";
static Y_MISSING: &str = "y is missing";
static Y_TYPEERROR: &str = "y is not of type Integer";

/// Represents the map that a game is played on
#[derive(Clone, Debug)]
pub struct Map {
    hexes: Vec<Vec<Option<Hex>>>,
}

impl Map {
    /// Parses a `Map` from the TOML data in `s`
    pub fn from_toml(s: &str) -> Self {
        let toml = s.parse::<Value>().expect(INVALID_TOML);
        let width = toml
            .get("width")
            .expect(WIDTH_MISSING)
            .as_integer()
            .expect(WIDTH_TYPEERROR);
        let height = toml
            .get("height")
            .expect(HEIGHT_MISSING)
            .as_integer()
            .expect(HEIGHT_TYPEERROR);
        let mut hexes = vec![vec![None; height as usize]; width as usize];
        let hexes_toml = toml.get("hexes").expect(HEXES_MISSING);
        for value in hexes_toml.as_array().expect(HEXES_TYPEERROR) {
            let hex = Hex::from_toml(value);
            let x = value
                .get("x")
                .expect(X_MISSING)
                .as_integer()
                .expect(X_TYPEERROR);
            let y = value
                .get("y")
                .expect(Y_MISSING)
                .as_integer()
                .expect(Y_TYPEERROR);
            if hexes[x as usize][y as usize].is_some() {
                panic!("hex at x={}, y={} is not empty", x, y);
            }
            hexes[x as usize][y as usize] = Some(hex);
        }
        Self { hexes }
    }
}
