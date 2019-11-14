use crate::economy::{ParTrack, Player, PubCom, Shares, StockChart};
use crate::geography::{Map, TileSet};
use crate::rounds::{OperatingRound, PrivAuction, StockRound};
use crate::{PhaseId, PrivComId, PubComId, RoundId, TrainSet};
use std::collections::HashMap;
use std::fs;

static ACTION_FORBIDDEN: &str = "action is forbidden";

/// Represents a game
#[derive(Debug)]
pub struct Game {
    round: RoundId,
    phase: PhaseId,
    players: Vec<Player>,
    current_player: usize,
    priority_player: usize,
    passes: usize,
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

    /// Returns whether placing a bid of `amount` on `private` is allowed
    pub fn bid_priv_allowed(&self, private: PrivComId, amount: u32) -> bool {
        if let RoundId::PrivAuction(priv_auction) = &self.round {
            if let Some(current_priv) = priv_auction.current() {
                let current_player = &self.players[self.current_player];
                priv_auction.can_afford_bid(&current_player, private, amount)
                    && amount + 5 >= priv_auction.max_bid(private)
                    && ((private == current_priv
                        && priv_auction
                            .bids(&current_player)
                            .get(&private)
                            .map_or(false, |&a| a != priv_auction.max_bid(private)))
                        || (private != current_priv
                            && current_priv.cost() == priv_auction.max_bid(current_priv)
                            && !priv_auction
                                .bids(&self.players[self.current_player])
                                .contains_key(&private)))
            } else {
                false
            }
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
            self.passes = 0;
            priv_auction.insert_bid(&self.players[self.current_player], private, amount);
            priv_auction.reset_non_max_bids(private);
        } else {
            unreachable!();
        }
        self.advance_current_player();
    }

    /// Returns whether buying the cheapest private company is allowed
    pub fn buy_cheapest_priv_allowed(&self) -> bool {
        if let RoundId::PrivAuction(priv_auction) = &self.round {
            priv_auction
                .current_if_buy_allowed(&self.players[self.current_player])
                .is_some()
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
            let current_player = &mut self.players[self.current_player];
            if let Some(current_priv) = priv_auction.current_if_buy_allowed(&current_player) {
                self.passes = 0;
                priv_auction.advance_current();
                current_player.buy_priv(current_priv, current_priv.cost());
                self.priority_player = (self.current_player + 1) % self.players.len();
                if priv_auction.current().is_none() {
                    self.enter_first_stock_round();
                    return;
                }
            }
        } else {
            unreachable!();
        }
        self.advance_current_player();
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
                    // TODO
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

    /// Returns whether buying a share of `pub_com` from the IPO is allowed
    pub fn buy_ipo_share_allowed(&self, pub_com: PubComId) -> bool {
        if let RoundId::StockRound(_) = &self.round {
            if let Some(par) = self.par_track.value(pub_com) {
                let current_player = &self.players[self.current_player];
                self.ipo.count(pub_com) > 0
                    && current_player.shares().count(pub_com) < 6
                    && self.certificate_count(current_player) < self.certificate_limit()
                    && current_player.capital() >= par
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
            stock_round.insert_pub_com_sold(pub_com, current_player);
            stock_round.set_action_performed();
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

    fn enter_first_stock_round(&mut self) {
        self.round = RoundId::StockRound(StockRound::new(false));
        self.current_player = self.priority_player;
    }

    fn enter_first_operating_round(&mut self) {
        self.round =
            RoundId::OperatingRound(OperatingRound::new(self.phase.operating_round_count() - 1))
    }

    fn operate_priv_coms(&mut self) {
        for player in &mut self.players {
            player.operate_priv_coms();
        }
    }

    fn advance_current_player(&mut self) {
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

    fn update_president(&mut self, pub_com: PubComId) {
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
            .map(|&p| {
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
