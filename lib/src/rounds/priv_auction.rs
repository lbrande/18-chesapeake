use crate::economy::Player;
use crate::PrivComId;
use std::collections::HashMap;

#[derive(Debug)]
/// Represents the auction for private companies
pub struct PrivAuction {
    current_priv: Option<PrivComId>,
    bids: Vec<HashMap<PrivComId, u32>>,
}

impl PrivAuction {
    pub(crate) fn new(player_count: usize) -> Self {
        PrivAuction {
            current_priv: Some(PrivComId::DAndR(20)),
            bids: vec![HashMap::new(); player_count],
        }
    }

    pub(crate) fn done(&self) -> bool {
        self.current_priv.is_none()
    }

    pub(crate) fn in_auction(&self) -> bool {
        if let Some(current_priv) = self.current_priv {
            current_priv.cost() != self.max_bid(current_priv)
        } else {
            false
        }
    }

    pub(crate) fn next_player_in_auction(&self) -> Option<usize> {
        if let Some(current_priv) = self.current_priv {
            if current_priv.cost() == self.max_bid(current_priv) {
                None
            } else {
                if let Some(player) = self.player_with_max_bid(current_priv) {
                    for i in 1..self.bids.len() {
                        if self.bids[(player + i) % self.bids.len()]
                            .get(&current_priv)
                            .is_some()
                        {
                            return Some(i);
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
        if let Some(current_priv) = self.current_priv {
            if current_priv.cost() == self.max_bid(current_priv)
                && self.can_afford_bid(player, current_priv, current_priv.cost())
            {
                Some(current_priv)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn current_if_pass_allowed(&self, player: &Player) -> Option<PrivComId> {
        if let Some(current_priv) = self.current_priv {
            if current_priv.cost() != 0
                && (current_priv.cost() == self.max_bid(current_priv)
                    || self.bids[player.id()].contains_key(&current_priv))
            {
                Some(current_priv)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn bids(&self, player: &Player) -> &HashMap<PrivComId, u32> {
        &self.bids[player.id()]
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

    pub(crate) fn advance_current_priv(&mut self) {
        if let Some(current_priv) = self.current_priv {
            self.current_priv = PrivComId::values()
                .find(|p| p.cost() > current_priv.cost())
                .cloned();
        }
    }

    pub(crate) fn reduce_d_and_r_price(&mut self, cost: u32) {
        self.current_priv = Some(PrivComId::DAndR(cost - 5));
    }

    /// Returns the current private company of this `PrivAuction`
    pub fn current_priv(&self) -> Option<PrivComId> {
        self.current_priv
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
