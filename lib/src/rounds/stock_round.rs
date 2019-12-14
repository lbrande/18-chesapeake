use crate::economy::{Player, PubCom};
use crate::{Game, PubComId, RoundId};
use std::collections::HashSet;

static ACTION_FORBIDDEN: &str = "action is forbidden";

/// Represents a stock round
#[derive(Clone, Debug)]
pub struct StockRound {
    action_performed: bool,
    sell_allowed: bool,
    pub_coms_sold: HashSet<(PubComId, usize)>,
}

impl StockRound {
    pub(crate) fn new(sell_allowed: bool) -> Self {
        StockRound {
            action_performed: false,
            sell_allowed,
            pub_coms_sold: HashSet::new(),
        }
    }

    pub(crate) fn insert_pub_com_sold(&mut self, pub_com: PubComId, player: &Player) {
        self.pub_coms_sold.insert((pub_com, player.id()));
    }

    pub(crate) fn set_action_performed(&mut self) {
        self.action_performed = true;
    }

    pub(crate) fn unset_action_performed(&mut self) {
        self.action_performed = false;
    }

    /// Returns whether the current player has performed an action
    pub fn action_performed(&self) -> bool {
        self.action_performed
    }

    /// Returns whether selling shares is allowed in this `StockRound`
    pub fn sell_allowed(&self) -> bool {
        self.sell_allowed
    }
}

impl Game {
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
}
