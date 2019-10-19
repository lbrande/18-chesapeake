use crate::economy::Player;
use crate::PrivComId;
use std::collections::HashMap;

#[derive(Debug)]
/// Represents the auction for private companies
pub struct PrivAuction {
    current: Option<PrivComId>,
    bids: Vec<HashMap<PrivComId, u32>>,
}

impl PrivAuction {
    pub(crate) fn new(player_count: usize) -> Self {
        PrivAuction {
            current: Some(PrivComId::DAndR(20)),
            bids: vec![HashMap::new(); player_count],
        }
    }

    pub(crate) fn next_player_in_auction(&self) -> Option<usize> {
        if let Some(current) = self.current {
            if current.cost() == self.max_bid(current) {
                None
            } else {
                if let Some(player) = self.player_with_max_bid(current) {
                    for i in 1..self.bids.len() {
                        let j = (player + i) % self.bids.len();
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

    pub(crate) fn current_if_buy_allowed(&self, player: &Player) -> Option<PrivComId> {
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

    pub(crate) fn current_if_pass_allowed(&self, player: &Player) -> Option<PrivComId> {
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

    pub(crate) fn max_bid(&self, private: PrivComId) -> u32 {
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

    pub(crate) fn only_bid(&self, private: PrivComId) -> Option<(usize, u32)> {
        let mut only_bid = None;
        for i in 0..self.bids.len() {
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

    pub(crate) fn can_afford_bid(&self, player: &Player, private: PrivComId, amount: u32) -> bool {
        let previous_total_amount: u32 = self.bids[player.id()].iter().map(|(_, a)| a).sum();
        if let Some(&bid) = self.bids[player.id()].get(&private) {
            amount + previous_total_amount - bid <= player.capital()
        } else {
            amount + previous_total_amount <= player.capital()
        }
    }

    pub(crate) fn insert_bid(&mut self, player: &Player, private: PrivComId, amount: u32) {
        self.bids[player.id()].insert(private, amount);
    }

    pub(crate) fn remove_bid(&mut self, player: &Player, private: PrivComId) {
        self.bids[player.id()].remove(&private);
    }

    pub(crate) fn reset_non_max_bids(&mut self, private: PrivComId) {
        for bids in &mut self.bids {
            if bids.contains_key(&private) {
                bids.insert(private, 0);
            }
        }
    }

    pub(crate) fn advance_current(&mut self) {
        if let Some(current) = self.current {
            self.current = PrivComId::values()
                .find(|p| p.cost() > current.cost())
                .cloned();
        }
    }

    pub(crate) fn reduce_d_and_r_price(&mut self, cost: u32) {
        self.current = Some(PrivComId::DAndR(cost - 5));
    }

    /// Returns the current private company of this `PrivAuction`
    pub fn current(&self) -> Option<PrivComId> {
        self.current
    }

    /// Returns the bids of `player` in this `PrivAuction`
    pub fn bids(&self, player: &Player) -> &HashMap<PrivComId, u32> {
        &self.bids[player.id()]
    }

    fn player_with_max_bid(&self, private: PrivComId) -> Option<usize> {
        let mut player = None;
        let mut max_bid = private.cost();
        for i in 0..self.bids.len() {
            if let Some(&bid) = self.bids[i].get(&private) {
                if bid > max_bid {
                    player = Some(i);
                    max_bid = bid;
                }
            }
        }
        player
    }
}
