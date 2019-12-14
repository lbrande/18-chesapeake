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
    round: RoundId,
    phase: PhaseId,
    players: Vec<Player>,
    player_count: usize,
    priority_player: usize,
    pub_coms: HashMap<PubComId, PubCom>,
    map: Map,
    tile_set: TileSet,
    train_set: TrainSet,
    stock_chart: StockChart,
    par_track: ParTrack,
    ipo: Shares,
    pool: Shares,
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
            player_count,
            priority_player: 0,
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

    /// Returns whether placing a bid of `amount` on `private` is allowed
    pub fn bid_priv_allowed(&self, private: PrivComId, amount: u32) -> bool {
        if let RoundId::PrivAuction(priv_auction) = &self.round {
            priv_auction.bid_allowed(
                &self.players[priv_auction.current_player()],
                private,
                amount,
            )
        } else {
            false
        }
    }

    /// Places a bid of `amount` on `private`
    pub fn bid_priv(&mut self, private: PrivComId, amount: u32) {
        if !self.bid_priv_allowed(private, amount) {
            panic!(ACTION_FORBIDDEN);
        }
        if let RoundId::PrivAuction(priv_auction) = &mut self.round {
            priv_auction.bid(private, amount);
        } else {
            unreachable!();
        }
    }

    /// Returns whether buying the cheapest private company is allowed
    pub fn buy_cheapest_priv_allowed(&self) -> bool {
        if let RoundId::PrivAuction(priv_auction) = &self.round {
            priv_auction.buy_cheapest_allowed(&self.players[priv_auction.current_player()])
        } else {
            false
        }
    }

    /// Buys the cheapest private company
    pub fn buy_cheapest_priv(&mut self) {
        if !self.buy_cheapest_priv_allowed() {
            panic!(ACTION_FORBIDDEN);
        }
        if let RoundId::PrivAuction(priv_auction) = &mut self.round {
            if priv_auction.buy_cheapest(
                &mut self.priority_player,
                &mut self.players[priv_auction.current_player()],
            ) {
                self.enter_first_stock_round();
                return;
            }
        } else {
            unreachable!();
        }
    }

    /// Returns whether passing is allowed
    pub fn pass_allowed(&self) -> bool {
        match &self.round {
            RoundId::StockRound(stock_round) => stock_round
                .pass_allowed(self.certificate_count(&self.players[stock_round.current_player()])),
            RoundId::PrivAuction(priv_auction) => {
                priv_auction.pass_allowed(&self.players[priv_auction.current_player()])
            }
            _ => false,
        }
    }

    /// Passes
    pub fn pass(&mut self) {
        if !self.pass_allowed() {
            panic!(ACTION_FORBIDDEN);
        }
        match &mut self.round {
            RoundId::StockRound(stock_round) => {
                if stock_round.pass() {
                    self.enter_first_operating_round();
                }
            }
            RoundId::PrivAuction(priv_auction) => {
                if priv_auction.pass(&mut self.players) {
                    self.enter_first_stock_round();
                    return;
                }
            }
            _ => unreachable!(),
        }
    }

    /// Returns whether buying a share of `pub_com` from the IPO is allowed
    pub fn buy_ipo_share_allowed(&self, pub_com: PubComId) -> bool {
        if let RoundId::StockRound(stock_round) = &self.round {
            if let Some(par) = self.par_track.value(pub_com) {
                let current_player = &self.players[stock_round.current_player()];
                stock_round.buy_ipo_share_allowed(current_player, self.ipo.count(pub_com), self.certificate_count(current_player), par, pub_com)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Buys a share of `pub_com` from the IPO
    pub fn buy_ipo_share(&mut self, pub_com: PubComId) {
        if !self.buy_ipo_share_allowed(pub_com) {
            panic!(ACTION_FORBIDDEN);
        }
        if let RoundId::StockRound(_) = &self.round {
            if let Some(par) = self.par_track.value(pub_com) {
                let current_player = &mut self.players[self.current_player];
                self.ipo.remove_shares(pub_com, 1);
                current_player.shares_mut().add_shares(pub_com, 1);
                current_player.remove_capital(par);
                self.update_president(pub_com);
                if self.ipo.count(pub_com) < 5 {
                    self.pub_coms
                        .insert(pub_com, PubCom::new(pub_com, 10 * par));
                    self.map.place_home_station(pub_com);
                }
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
        self.end_turn();
    }

    /// Returns whether buying a share of `pub_com` from the bank pool is allowed
    pub fn buy_pool_share_allowed(&self, pub_com: PubComId) -> bool {
        if let RoundId::StockRound(_) = &self.round {
            if let Some(value) = self.stock_chart.value(pub_com) {
                let current_player = &self.players[self.current_player];
                self.pool.count(pub_com) > 0
                    && current_player.shares().count(pub_com) < 6
                    && self.certificate_count(current_player) < self.certificate_limit()
                    && current_player.capital() >= value
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Buys a share of `pub_com` from the bank pool
    pub fn buy_pool_share(&mut self, pub_com: PubComId) {
        if !self.buy_pool_share_allowed(pub_com) {
            panic!(ACTION_FORBIDDEN);
        }
        if let RoundId::StockRound(_) = &self.round {
            if let Some(value) = self.stock_chart.value(pub_com) {
                let current_player = &mut self.players[self.current_player];
                self.pool.remove_shares(pub_com, 1);
                current_player.shares_mut().add_shares(pub_com, 1);
                current_player.remove_capital(value);
                self.update_president(pub_com);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
        self.end_turn();
    }

    /// Returns whether buying the precidency of `pub_com`, setting the par value to `par` is allowed
    pub fn buy_presidency_allowed(&self, pub_com: PubComId, par: u32) -> bool {
        if let RoundId::StockRound(_) = &self.round {
            let current_player = &self.players[self.current_player];
            self.par_track.values().contains(&par)
                && self.ipo.contains_presidency(pub_com)
                && self.certificate_count(current_player) < self.certificate_limit()
                && current_player.capital() >= par * 2
        } else {
            false
        }
    }

    /// Buys the precidency of `pub_com`, setting the par value to `par`
    pub fn buy_presidency(&mut self, pub_com: PubComId, par: u32) {
        if !self.buy_presidency_allowed(pub_com, par) {
            panic!(ACTION_FORBIDDEN);
        }
        if let RoundId::StockRound(_) = &self.round {
            let current_player = &mut self.players[self.current_player];
            self.pool.remove_shares(pub_com, 2);
            self.pool.remove_presidency(pub_com);
            current_player.shares_mut().add_shares(pub_com, 2);
            current_player.shares_mut().add_presidency(pub_com);
            current_player.remove_capital(par * 2);
            self.par_track.add_token(pub_com, par);
            self.stock_chart.add_token(pub_com, par);
        } else {
            unreachable!();
        }
        self.end_turn();
    }

    /// Returns whether ending the turn is allowed
    pub fn end_turn_allowed(&self) -> bool {
        if let RoundId::StockRound(stock_round) = &self.round {
            stock_round.action_performed()
                && self.certificate_count(&self.players[self.current_player])
                    <= self.certificate_limit()
        } else {
            false
        }
    }

    /// Ends the turn
    pub fn end_turn(&mut self) {
        if !self.end_turn_allowed() {
            panic!(ACTION_FORBIDDEN);
        }
        if let RoundId::StockRound(stock_round) = &mut self.round {
            stock_round.unset_action_performed();
            self.passes = 0;
        } else {
            unreachable!();
        }
        self.advance_current_player();
    }

    /// Returns whether selling `count` shares of `pub_com` is allowed
    pub fn sell_shares_allowed(&self, pub_com: PubComId, count: u32) -> bool {
        if let RoundId::StockRound(stock_round) = &self.round {
            let current_player = &self.players[self.current_player];
            let owned_count = current_player.shares().count(pub_com);
            let president = current_player.shares().contains_presidency(pub_com);
            (!president
                || owned_count - count >= 2
                || self
                    .players
                    .iter()
                    .any(|p| p.id() != current_player.id() && p.shares().count(pub_com) >= 2))
                && count + self.pool.count(pub_com) <= 5
                && owned_count >= count
                && self.stock_chart.value(pub_com).is_some()
                && stock_round.sell_allowed()
        } else {
            false
        }
    }

    /// Sells `count` shares of `pub_com`
    pub fn sell_shares(&mut self, pub_com: PubComId, count: u32) {
        if !self.sell_shares_allowed(pub_com, count) {
            panic!(ACTION_FORBIDDEN);
        }
        if let RoundId::StockRound(stock_round) = &mut self.round {
            let current_player = &mut self.players[self.current_player];
            stock_round.set_action_performed();
            stock_round.insert_pub_com_sold(pub_com, current_player);
            current_player.shares_mut().remove_shares(pub_com, count);
            self.pool.add_shares(pub_com, count);
            current_player.add_capital(self.stock_chart.value(pub_com).unwrap() * count);
            self.stock_chart.move_down(pub_com, count as usize);
            self.update_president(pub_com);
        } else {
            unreachable!();
        }
        self.advance_current_player();
    }

    pub(crate) fn operate_priv_coms(players: &mut [Player]) {
        for player in players {
            player.operate_priv_coms();
        }
    }

    /// Returns the current player of this game
    pub fn current_player(&self) -> usize {
        match &mut self.round {
            RoundId::StockRound(stock_round) => stock_round.current_player(),
            RoundId::PrivAuction(priv_auction) => priv_auction.current_player(),
            RoundId::OperatingRound(operating_round) => {
                self.president(self.current_pub_com()).unwrap()
            }
            _ => unreachable!(),
        }
    }

    /// Returns the certificate limit of this `Game`
    pub fn certificate_limit(player_count: usize) -> u32 {
        match player_count {
            2 => 20,
            3 => 20,
            4 => 16,
            5 => 13,
            6 => 11,
            _ => unreachable!(),
        }
    }

    fn enter_first_stock_round(&mut self) {
        self.round = RoundId::StockRound(StockRound::new(
            false,
            self.player_count,
            self.priority_player,
        ));
    }

    fn enter_first_operating_round(&mut self) {
        self.round = RoundId::OperatingRound(OperatingRound::new(
            self.phase.operating_round_count() - 1,
            PubComId::values()
                .filter(|&p| self.stock_chart.value(p).is_some())
                .collect(),
        ));
        Game::operate_priv_coms(&mut self.players);
        self.current_player = self.president(self.current_pub_com()).unwrap();
    }

    fn update_president(&mut self, pub_com: PubComId) {
        if let Some(president) = self.president(pub_com) {
            let mut new_president = president;
            let mut max_shares = self.players[president].shares().count(pub_com);
            for i in 1..self.player_count {
                let j = (president + i) % self.player_count;
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

    fn president(&self, pub_com: PubComId) -> Option<usize> {
        for player in &self.players {
            if player.shares().contains_presidency(pub_com) {
                return Some(player.id());
            }
        }
        None
    }

    fn certificate_count(&self, player: &Player) -> u32 {
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
}

fn read_toml_file(name: &str) -> String {
    fs::read_to_string(format!("conf/{}.toml", name)).unwrap()
}
