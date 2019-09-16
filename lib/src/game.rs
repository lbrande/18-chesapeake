use crate::economy::{ParTrack, Player, PrivateAuction, PublicCompany, Shares, StockChart};
use crate::geography::{Map, TileSet};
use crate::{PhaseId, PrivComId, PubComId, RoundId, TrainSet};
use std::collections::HashMap;
use std::fs::read_to_string;

/// Represents a game
#[derive(Clone, Debug)]
pub struct Game {
    phase: PhaseId,
    round: RoundId,
    players: Vec<Player>,
    publics: HashMap<PubComId, PublicCompany>,
    private_auction: PrivateAuction,
    map: Map,
    tile_set: TileSet,
    train_set: TrainSet,
    stock_chart: StockChart,
    par_track: ParTrack,
    ipo: Shares,
    bank_pool: Shares,
    bank_amount: u32,
}

impl Game {
    /// Returns a game
    pub fn new(player_count: usize) -> Self {
        if player_count < 2 || player_count > 6 {
            panic!("player_count out of bounds");
        }
        Self {
            phase: PhaseId::Phase2,
            round: RoundId::PrivAuction,
            players: vec![Player::new(2400 / player_count as u32); player_count],
            publics: HashMap::new(),
            private_auction: PrivateAuction::new(player_count),
            map: Map::from_toml(&read_toml_file("map")),
            tile_set: TileSet::from_toml(&read_toml_file("tile_set")),
            train_set: TrainSet::from_toml(&read_toml_file("train_set")),
            stock_chart: StockChart::from_toml(&read_toml_file("stock_chart")),
            par_track: ParTrack::from_toml(&read_toml_file("par_track")),
            ipo: Shares::ipo_shares(),
            bank_pool: Shares::bank_pool_shares(),
            bank_amount: 8000,
        }
    }

    /// Places a bid on a private company
    pub fn place_bid(&mut self, player: usize, private: PrivComId, amount: u32) -> bool {
        self.private_auction
            .place_bid(self.players[player].capital(), player, private, amount)
    }

    /// Buys the current (cheapest) private company
    pub fn buy_current(&mut self, player: usize) -> bool {
        if let Some(private) = self
            .private_auction
            .buy_current(self.players[player].capital(), player)
        {
            self.players[player].buy_private(private, private.get_cost());
            true
        } else {
            false
        }
    }
}

fn read_toml_file(name: &str) -> String {
    read_to_string(format!("conf/{}.toml", name)).unwrap()
}
