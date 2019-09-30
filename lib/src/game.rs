use crate::economy::{ParTrack, Player, PrivAuction, PubCom, Shares, StockChart, StockRound};
use crate::geography::{Map, TileSet};
use crate::{PhaseId, PrivComId, PubComId, RoundId, TrainSet};
use std::collections::HashMap;
use std::fs::read_to_string;

/// Represents a game
#[derive(Clone, Debug)]
pub struct Game {
    phase: PhaseId,
    round: RoundId,
    current: usize,
    priority: usize,
    players: Vec<Player>,
    pub_coms: HashMap<PubComId, PubCom>,
    priv_auction: PrivAuction,
    stock_round: StockRound,
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
            priority: 0,
            players,
            pub_coms: HashMap::new(),
            priv_auction: PrivAuction::new(player_count),
            stock_round: StockRound::new(false),
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
    pub fn place_bid(&mut self, private: PrivComId, amount: u32) {
        if self.round == RoundId::StockRound {
            self.priv_auction
                .place_bid(&self.players[self.current], private, amount);
            self.advance_current_in_priv_auction();
        }
    }

    /// Returns whether the specified bid is allowed
    pub fn bid_allowed(&self, private: PrivComId, amount: u32) -> bool {
        self.priv_auction
            .bid_allowed(&self.players[self.current], private, amount)
    }

    /// Buys the current (cheapest) private company
    pub fn buy_current(&mut self) {
        if let Some(private) = self.priv_auction.buy_current(&self.players[self.current]) {
            self.players[self.current].buy_priv(private, private.cost());
            self.priority = (self.current + 1) % self.players.len();
            self.advance_current_in_priv_auction();
            self.enter_stock_round_if_priv_auction_is_done();
        }
    }

    /// Returns whether buying the current (cheapest) private company is allowed
    pub fn buy_allowed(&self) -> bool {
        self.priv_auction.buy_allowed(&self.players[self.current])
    }

    /// Passes
    pub fn pass(&mut self) {
        if self.round == RoundId::StockRound {
            //TODO
        } else if self.round == RoundId::PrivAuction {
            if self.priv_auction.in_auction() {
                if let Some((private, player_id, price)) =
                    self.priv_auction.pass_auction(&self.players[self.current])
                {
                    self.players[player_id].buy_priv(private, price);
                    self.advance_current_in_priv_auction();
                    self.enter_stock_round_if_priv_auction_is_done();
                } else {
                    self.advance_current_in_priv_auction();
                }
            } else {
                if self
                    .priv_auction
                    .pass_current(&self.players[self.current], self.players.len())
                {
                    self.operate_priv_coms();
                }
                self.advance_current();
            }
        }
    }

    /// Returns whether passing is allowed
    pub fn pass_allowed(&self) -> bool {
        self.round == RoundId::StockRound
            || self.priv_auction.pass_allowed(&self.players[self.current])
    }

    fn advance_current(&mut self) {
        self.current = (self.current + 1) % self.players.len();
    }

    fn advance_current_in_priv_auction(&mut self) {
        if let Some(player_id) = self.priv_auction.next_player_in_auction(self.players.len()) {
            self.current = player_id;
        } else {
            self.advance_current();
        }
    }

    fn operate_priv_coms(&mut self) {
        for player in &mut self.players {
            player.operate_priv_coms();
        }
    }

    fn enter_stock_round_if_priv_auction_is_done(&mut self) {
        if self.priv_auction.is_done() {
            self.round = RoundId::StockRound;
            self.current = self.priority;
        }
    }
}

fn read_toml_file(name: &str) -> String {
    read_to_string(format!("conf/{}.toml", name)).unwrap()
}
