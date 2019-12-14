use crate::economy::Player;
use crate::{Game, PrivComId};
use std::collections::HashMap;

#[derive(Debug)]
/// Represents the auction for private companies
pub struct PrivAuction {
    current: Option<PrivComId>,
    bids: Vec<HashMap<PrivComId, u32>>,
    player_count: usize,
    current_player: usize,
    passes: usize,
}

impl PrivAuction {
    pub(crate) fn new(player_count: usize) -> Self {
        PrivAuction {
            current: Some(PrivComId::DAndR(20)),
            player_count,
            bids: vec![HashMap::new(); player_count],
            current_player: 0,
            passes: 0,
        }
    }

    pub(crate) fn bid_allowed(&self, player: &Player, private: PrivComId, amount: u32) -> bool {
        if let Some(current_priv) = self.current() {
            self.can_afford_bid(player, private, amount)
                && amount + 5 >= self.max_bid(private)
                && ((private == current_priv
                    && self.bids[self.current_player]
                        .get(&private)
                        .map_or(false, |&a| a != self.max_bid(private)))
                    || (private != current_priv
                        && current_priv.cost() == self.max_bid(current_priv)
                        && !self.bids[self.current_player].contains_key(&private)))
        } else {
            false
        }
    }

    pub(crate) fn bid(&mut self, private: PrivComId, amount: u32) {
        self.passes = 0;
        self.bids[self.current_player].insert(private, amount);
        for bids in &mut self.bids {
            if bids.contains_key(&private) {
                bids.insert(private, 0);
            }
        }
        self.advance_current_player();
    }

    pub(crate) fn buy_cheapest_allowed(&self, player: &Player) -> bool {
        self.current_if_buy_allowed(player).is_some()
    }

    pub(crate) fn buy_cheapest(&mut self, priority_player: &mut usize, player: &mut Player) -> bool {
        self.passes = 0;
        *priority_player = (self.current_player + 1) % self.player_count;
        if let Some(current_priv) = self.current_if_buy_allowed(player) {
            player.buy_priv(current_priv, current_priv.cost());
        }
        self.advance_current();
        self.advance_current_player();
        self.current().is_none()
    }

    pub(crate) fn pass_allowed(&self, player: &Player) -> bool {
        self.current_if_pass_allowed(player)
            .is_some()
    }

    pub(crate) fn pass(&mut self, players: &mut [Player]) -> bool {
        if let Some(current_priv) = self.current_if_pass_allowed(&players[self.current_player]) {
            if self.max_bid(current_priv) != current_priv.cost() {
                self.passes = 0;
                self.bids[self.current_player].remove(&current_priv);
                if let Some((player, amount)) = self.only_bid(current_priv) {
                    self.advance_current();
                    players[player].buy_priv(current_priv, amount);
                }
            } else {
                self.passes += 1;
                if self.passes == self.player_count {
                    self.passes = 0;
                    if let PrivComId::DAndR(cost) = current_priv {
                        self.current = Some(PrivComId::DAndR(cost - 5));
                    } else {
                        Game::operate_priv_coms(players);
                    }
                }
            }
        }
        self.advance_current_player();
        self.current().is_none()
    }

    pub(crate) fn current_player(&self) -> usize {
        self.current_player
    }

    /// Returns the current private company in this `PrivAuction`
    pub fn current(&self) -> Option<PrivComId> {
        self.current
    }

    /// Returns the bids of `player` in this `PrivAuction`
    pub fn bids(&self, player: &Player) -> &HashMap<PrivComId, u32> {
        &self.bids[player.id()]
    }

    fn advance_current(&mut self) {
        if let Some(current) = self.current {
            self.current = PrivComId::values().find(|p| p.cost() > current.cost());
        }
    }

    fn advance_current_player(&mut self) {
        if let Some(player) = self.next_player_in_auction() {
            self.current_player = player;
        } else {
            self.current_player = (self.current_player + 1) % self.player_count;
        }
    }

    fn next_player_in_auction(&self) -> Option<usize> {
        if let Some(current) = self.current {
            if current.cost() == self.max_bid(current) {
                None
            } else {
                if let Some(player) = self.player_with_max_bid(current) {
                    for i in 1..self.player_count {
                        let j = (player + i) % self.player_count;
                        if self.bids[j].get(&current).is_some() {
                            return Some(j);
                        }
                    }
                }
                None
            }
        } else {
            None
        }
    }

    fn current_if_buy_allowed(&self, player: &Player) -> Option<PrivComId> {
        if let Some(current) = self.current {
            if current.cost() == self.max_bid(current)
                && self.can_afford_bid(player, current, current.cost())
            {
                Some(current)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn current_if_pass_allowed(&self, player: &Player) -> Option<PrivComId> {
        if let Some(current) = self.current {
            if current.cost() != 0
                && (current.cost() == self.max_bid(current)
                    || self.bids[player.id()].contains_key(&current))
            {
                Some(current)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn can_afford_bid(&self, player: &Player, private: PrivComId, amount: u32) -> bool {
        let previous_total_amount: u32 = self.bids[player.id()].iter().map(|(_, a)| a).sum();
        if let Some(&bid) = self.bids[player.id()].get(&private) {
            amount + previous_total_amount - bid <= player.capital()
        } else {
            amount + previous_total_amount <= player.capital()
        }
    }

    fn max_bid(&self, private: PrivComId) -> u32 {
        let mut max_bid = private.cost();
        for bids in &self.bids {
            if let Some(&bid) = bids.get(&private) {
                if bid > max_bid {
                    max_bid = bid;
                }
            }
        }
        max_bid
    }

    fn player_with_max_bid(&self, private: PrivComId) -> Option<usize> {
        let mut player = None;
        let mut max_bid = private.cost();
        for i in 0..self.player_count {
            if let Some(&bid) = self.bids[i].get(&private) {
                if bid > max_bid {
                    player = Some(i);
                    max_bid = bid;
                }
            }
        }
        player
    }

    fn only_bid(&self, private: PrivComId) -> Option<(usize, u32)> {
        let mut only_bid = None;
        for i in 0..self.player_count {
            if let Some(&bid) = self.bids[i].get(&private) {
                if only_bid.is_none() {
                    only_bid = Some((i, bid));
                } else {
                    return None;
                }
            }
        }
        only_bid
    }
}
