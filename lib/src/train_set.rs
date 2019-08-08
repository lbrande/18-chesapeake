use crate::TrainId;
use crate::INVALID_TOML;
use std::collections::HashSet;
use toml::Value;

static TRAINS_MISSING: &str = "trains is missing";
static TRAINS_TYPEERROR: &str = "trains is not of type Array";
static TRAIN_MISSING: &str = "train is missing";
static TRAIN_TYPEERROR: &str = "train is not of type String";
static COUNT_MISSING: &str = "count is missing";
static COUNT_TYPEERROR: &str = "count is not of type Integer";

/// Represents the trains and their respective counts avaliable in a game
#[derive(Clone, Debug)]
pub struct TrainSet {
    trains: HashSet<(TrainId, i32)>,
}

impl TrainSet {
    pub(crate) fn from_toml(s: &str) -> Self {
        let toml = s.parse::<Value>().expect(INVALID_TOML);
        let mut trains = HashSet::new();
        let trains_toml = toml.get("trains").expect(TRAINS_MISSING);
        for value in trains_toml.as_array().expect(TRAINS_TYPEERROR) {
            let train = value
                .get("train")
                .expect(TRAIN_MISSING)
                .as_str()
                .expect(TRAIN_TYPEERROR)
                .parse::<TrainId>()
                .unwrap();
            let count = value
                .get("count")
                .expect(COUNT_MISSING)
                .as_integer()
                .expect(COUNT_TYPEERROR);
            trains.insert((train, count as i32));
        }
        Self { trains }
    }
}
