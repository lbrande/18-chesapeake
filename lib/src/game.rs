use crate::economy::{ParTrack, Player, PrivateAuction, PublicCompany, Shares, StockChart};
use crate::geography::{Map, TileSet};
use crate::{PhaseId, PrivComId, PubComId, RoundId, TrainSet};
use std::collections::HashMap;
use std::fs::read_to_string;

macro_rules! current_player {
    ($self:ident) => { &$self.players[$self.current] }
}

/// Represents a game
#[derive(Clone, Debug)]
pub struct Game {
    phase: PhaseId,
    round: RoundId,
    current: usize,
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
        let mut players = Vec::with_capacity(player_count);
        for id in 0..player_count {
            players.push(Player::new(id, 2400 / player_count as u32));
        }
        Self {
            phase: PhaseId::Phase2,
            round: RoundId::PrivAuction,
            current: 0,
            players,
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
    pub fn place_bid(&mut self, private: PrivComId, amount: u32) -> bool {
        self.private_auction.place_bid(
            current_player!(self),
            private,
            amount,
        )
    }

    /// Buys the current (cheapest) private company
    pub fn buy_current(&mut self) -> bool {
        if let Some(private) = self.private_auction.buy_current(current_player!(self)) {
            self.players[self.current].buy_private(private, private.get_cost());
            true
        } else {
            false
        }
    }
}

fn read_toml_file(name: &str) -> String {
    read_to_string(format!("conf/{}.toml", name)).unwrap()
}
