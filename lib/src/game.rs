use crate::economy::{ParTrack, Player, PubCom, Shares, StockChart};
use crate::geography::{Map, TileSet};
use crate::rounds::{OperatingRound, PrivAuction, StockRound};
use crate::{PhaseId, PrivComId, PubComId, RoundId, TrainSet};
use std::collections::HashMap;
use std::{fs, u32};

static ACTION_FORBIDDEN: &str = "action is forbidden";

/// Represents a game
#[derive(Debug)]
pub struct Game {
    pub(crate) round: RoundId,
    pub(crate) phase: PhaseId,
    pub(crate) players: Vec<Player>,
    pub(crate) current_player: usize,
    pub(crate) priority_player: usize,
    pub(crate) passes: usize,
    pub(crate) pub_coms: HashMap<PubComId, PubCom>,
    pub(crate) map: Map,
    pub(crate) tile_set: TileSet,
    pub(crate) train_set: TrainSet,
    pub(crate) stock_chart: StockChart,
    pub(crate) par_track: ParTrack,
    pub(crate) ipo: Shares,
    pub(crate) pool: Shares,
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
            round: RoundId::PrivAuction(PrivAuction::new(player_count)),
            phase: PhaseId::Phase2,
            players,
            current_player: 0,
            priority_player: 0,
            passes: 0,
            pub_coms: HashMap::new(),
            map: Map::from_toml(&read_toml_file("map")),
            tile_set: TileSet::from_toml(&read_toml_file("tile_set")),
            train_set: TrainSet::from_toml(&read_toml_file("train_set")),
            stock_chart: StockChart::from_toml(&read_toml_file("stock_chart")),
            par_track: ParTrack::from_toml(&read_toml_file("par_track")),
            ipo: Shares::ipo_shares(),
            pool: Shares::empty_shares(),
        }
    }

    /// Returns whether passing is allowed
    pub fn pass_allowed(&self) -> bool {
        match &self.round {
            RoundId::StockRound(stock_round) => {
                !stock_round.action_performed()
                    && self.certificate_count(&self.players[self.current_player])
                        <= self.certificate_limit()
            }
            RoundId::PrivAuction(priv_auction) => priv_auction
                .current_if_pass_allowed(&self.players[self.current_player])
                .is_some(),
            _ => false,
        }
    }

    /// Passes
    pub fn pass(&mut self) {
        if !self.pass_allowed() {
            panic!(ACTION_FORBIDDEN);
        }
        match &mut self.round {
            RoundId::StockRound(_) => {
                self.passes += 1;
                if self.passes == self.players.len() {
                    self.passes = 0;
                    self.enter_first_operating_round();
                }
            }
            RoundId::PrivAuction(priv_auction) => {
                let current_player = &self.players[self.current_player];
                if let Some(current_priv) = priv_auction.current_if_pass_allowed(&current_player) {
                    if priv_auction.max_bid(current_priv) != current_priv.cost() {
                        self.passes = 0;
                        priv_auction.remove_bid(&current_player, current_priv);
                        if let Some((player, amount)) = priv_auction.only_bid(current_priv) {
                            priv_auction.advance_current();
                            self.players[player].buy_priv(current_priv, amount);
                            if priv_auction.current().is_none() {
                                self.enter_first_stock_round();
                                return;
                            }
                        }
                    } else {
                        self.passes += 1;
                        if self.passes == self.players.len() {
                            self.passes = 0;
                            if let PrivComId::DAndR(cost) = current_priv {
                                priv_auction.reduce_d_and_r_price(cost);
                            } else {
                                self.operate_priv_coms();
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        self.advance_current_player();
    }

    /// Returns the current player of this `Game`
    pub fn current_player(&self) -> usize {
        self.current_player
    }

    /// Returns the president of `pub_com` in this `Game`, if any
    pub fn president(&self, pub_com: PubComId) -> Option<usize> {
        for player in &self.players {
            if player.shares().contains_presidency(pub_com) {
                return Some(player.id());
            }
        }
        None
    }
    /// Returns the certificate limit of this `Game`
    pub fn certificate_limit(&self) -> u32 {
        match self.players.len() {
            2 => 20,
            3 => 20,
            4 => 16,
            5 => 13,
            6 => 11,
            _ => unreachable!(),
        }
    }

    /// Returns the certificate count of `player` in this `Game`
    pub fn certificate_count(&self, player: &Player) -> u32 {
        PubComId::values()
            .map(|p| {
                if let Some(value) = self.stock_chart.value(p) {
                    if value < 60 {
                        0
                    } else {
                        player.shares().count(p)
                    }
                } else {
                    player.shares().count(p)
                }
            })
            .sum()
    }

    pub(crate) fn advance_current_player(&mut self) {
        match &mut self.round {
            RoundId::StockRound(_) => {
                self.current_player = (self.current_player + 1) % self.players.len();
            }
            RoundId::PrivAuction(priv_auction) => {
                if let Some(player) = priv_auction.next_player_in_auction() {
                    self.current_player = player;
                } else {
                    self.current_player = (self.current_player + 1) % self.players.len();
                }
            }
            _ => (),
        }
    }

    pub(crate) fn enter_first_stock_round(&mut self) {
        self.round = RoundId::StockRound(StockRound::new(false));
        self.current_player = self.priority_player;
    }

    pub(crate) fn enter_first_operating_round(&mut self) {
        self.round = RoundId::OperatingRound(OperatingRound::new(
            self.phase.operating_round_count() - 1,
            PubComId::values()
                .filter(|&p| self.stock_chart.value(p).is_some())
                .collect(),
        ));
        self.operate_priv_coms();
        self.current_player = self.president(self.current_pub_com()).unwrap();
    }

    pub(crate) fn operate_priv_coms(&mut self) {
        for player in &mut self.players {
            player.operate_priv_coms();
        }
    }

    pub(crate) fn update_president(&mut self, pub_com: PubComId) {
        if let Some(president) = self.president(pub_com) {
            let mut new_president = president;
            let mut max_shares = self.players[president].shares().count(pub_com);
            for i in 1..self.players.len() {
                let j = (president + i) % self.players.len();
                let shares = self.players[j].shares().count(pub_com);
                if shares > max_shares {
                    new_president = j;
                    max_shares = shares;
                }
            }
            if new_president != president {
                self.players[president]
                    .shares_mut()
                    .remove_presidency(pub_com);
                self.players[new_president]
                    .shares_mut()
                    .add_presidency(pub_com);
            }
        }
    }

    fn current_pub_com(&self) -> PubComId {
        if let RoundId::OperatingRound(operating_round) = &self.round {
            operating_round
                .pub_coms_to_operate()
                .iter()
                .map(|&p| (p, self.stock_chart.value(p).unwrap()))
                .max_by(|(_, v_1), (_, v_2)| u32::cmp(v_1, v_2))
                .unwrap()
                .0
        } else {
            panic!(ACTION_FORBIDDEN);
        }
    }
}

fn read_toml_file(name: &str) -> String {
    fs::read_to_string(format!("conf/{}.toml", name)).unwrap()
}
