use crate::economy::{ParTrack, Player, PubCom, Shares, StockChart};
use crate::geography::{Map, TileSet};
use crate::rounds::{PrivAuction, StockRound};
use crate::{PhaseId, PrivComId, PubComId, RoundId, TrainSet};
use std::collections::HashMap;
use std::fs;

/// Represents a game
#[derive(Debug)]
pub struct Game {
    phase: PhaseId,
    round: RoundId,
    priority_player: usize,
    pub_coms: HashMap<PubComId, PubCom>,
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
            round: RoundId::PrivAuction(PrivAuction::new(players)),
            priority_player: 0,
            pub_coms: HashMap::new(),
            map: Map::from_toml(&read_toml_file("map")),
            tile_set: TileSet::from_toml(&read_toml_file("tile_set")),
            train_set: TrainSet::from_toml(&read_toml_file("train_set")),
            stock_chart: StockChart::from_toml(&read_toml_file("stock_chart")),
            par_track: ParTrack::from_toml(&read_toml_file("par_track")),
            ipo: Shares::ipo_shares(),
            bank_pool: Shares::empty_shares(),
            bank_amount: 8000,
        }
    }

    /// Places a bid on a private company
    pub fn place_bid(&mut self, private: PrivComId, amount: u32) {
        if let RoundId::PrivAuction(priv_auction) = &mut self.round {
            priv_auction.place_bid(private, amount);
        }
    }

    /// Returns whether the specified bid is allowed
    pub fn bid_allowed(&self, private: PrivComId, amount: u32) -> bool {
        if let RoundId::PrivAuction(priv_auction) = &self.round {
            priv_auction.bid_allowed(private, amount)
        } else {
            false
        }
    }

    /// Buys the current (cheapest) private company
    pub fn buy_current(&mut self) {
        if let RoundId::PrivAuction(priv_auction) = &mut self.round {
            if priv_auction.buy_current() {
                self.enter_first_stock_round();
            }
        }
    }

    /// Returns whether buying the current (cheapest) private company is allowed
    pub fn buy_allowed(&self) -> bool {
        if let RoundId::PrivAuction(priv_auction) = &self.round {
            priv_auction.buy_allowed()
        } else {
            false
        }
    }

    /// Passes
    pub fn pass(&mut self) {
        match &mut self.round {
            RoundId::StockRound(stock_round) => {
                stock_round.pass();
            }
            RoundId::PrivAuction(priv_auction) => {
                if priv_auction.pass() {
                    self.enter_first_stock_round();
                }
            }
            _ => (),
        }
    }

    /// Returns whether passing is allowed
    pub fn pass_allowed(&self) -> bool {
        if let RoundId::PrivAuction(priv_auction) = &self.round {
            priv_auction.pass_allowed()
        } else if let RoundId::StockRound(_) = self.round {
            true
        } else {
            false
        }
    }

    fn enter_first_stock_round(&mut self) {
        let (players, priority_player) = self.round.take().state();
        self.round = RoundId::StockRound(StockRound::new(players, priority_player, false));
    }

    pub(crate) fn operate_priv_coms(players: &mut Vec<Player>) {
        for player in players {
            player.operate_priv_coms();
        }
    }
}

fn read_toml_file(name: &str) -> String {
    fs::read_to_string(format!("conf/{}.toml", name)).unwrap()
}
