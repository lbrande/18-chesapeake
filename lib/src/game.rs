use crate::economy::{ParTrack, Player, PublicCompany, Shares, StockChart};
use crate::geography::{Map, TileSet};
use crate::*;
use std::collections::HashMap;
use std::fs::read_to_string;

/// Represents a game
#[derive(Clone, Debug)]
pub struct Game {
    players: Vec<Player>,
    publics: HashMap<PubComId, PublicCompany>,
    map: Map,
    tile_set: TileSet,
    train_set: TrainSet,
    stock_chart: StockChart,
    par_track: ParTrack,
    ipo: Shares,
    bank_pool: Shares,
}

impl Game {
    /// Returns a game
    pub fn new(player_count: u32) -> Self {
        if player_count < 2 || player_count > 6 {
            panic!("player_count out of bounds");
        }
        let mut players = Vec::new();
        for _ in 0..player_count {
            players.push(Player::new(2400 / player_count))
        }
        Self {
            players,
            publics: HashMap::new(),
            map: Map::from_toml(&read_toml_file("map")),
            tile_set: TileSet::from_toml(&read_toml_file("tile_set")),
            train_set: TrainSet::from_toml(&read_toml_file("train_set")),
            stock_chart: StockChart::from_toml(&read_toml_file("stock_chart")),
            par_track: ParTrack::from_toml(&read_toml_file("par_track")),
            ipo: Shares::ipo_shares(),
            bank_pool: Shares::bank_pool_shares(),
        }
    }
}

fn read_toml_file(name: &str) -> String {
    read_to_string(format!("conf/{}.toml", name)).unwrap()
}
