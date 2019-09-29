use crate::economy::Player;
use crate::PrivComId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Represents the auction for private companies
pub struct PrivAuction {
    current: Option<PrivComId>,
    bids: Vec<HashMap<PrivComId, u32>>,
    passes: usize,
}

impl PrivAuction {
    pub(crate) fn new(player_count: usize) -> Self {
        PrivAuction {
            current: Some(PrivComId::DAndR(20)),
            bids: vec![HashMap::new(); player_count],
            passes: 0,
        }
    }

    pub(crate) fn place_bid(&mut self, player: &Player, private: PrivComId, amount: u32) {
        if self.bid_allowed(player, private, amount) {
            self.passes = 0;
            self.bids[player.id()].insert(private, amount);
        }
    }

    pub(crate) fn bid_allowed(&self, player: &Player, private: PrivComId, amount: u32) -> bool {
        if let Some(current) = self.current {
            self.can_afford_bid(player, private, amount)
                && amount + 5 >= self.max_bid(private)
                && ((private == current
                    && self.bids[player.id()]
                        .get(&private)
                        .map_or(false, |&a| a != self.max_bid(private)))
                    || (private != current
                        && current.cost() == self.max_bid(current)
                        && !self.bids[player.id()].contains_key(&private)))
        } else {
            false
        }
    }

    pub(crate) fn buy_current(&mut self, player: &Player) -> Option<PrivComId> {
        if let Some(current) = self.current_if_buy_allowed(player) {
            self.passes = 0;
            self.current = PrivComId::values()
                .find(|p| p.cost() > current.cost())
                .cloned();
            Some(current)
        } else {
            None
        }
    }

    pub(crate) fn buy_allowed(&self, player: &Player) -> bool {
        self.current_if_buy_allowed(player).is_some()
    }

    /// Returns whether everyone has passed
    pub(crate) fn pass_current(&mut self, player: &Player, player_count: usize) -> bool {
        if let Some(current) = self.current_if_pass_allowed(player) {
            self.passes += 1;
            if self.passes == player_count {
                self.passes = 0;
                if let PrivComId::DAndR(cost) = current {
                    self.current = Some(PrivComId::DAndR(cost - 5));
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// If a private company was sold, returns the `PrivComId`, the player id and the price
    pub(crate) fn pass_auction(&mut self, player: &Player) -> Option<(PrivComId, usize, u32)> {
        if let Some(current) = self.current_if_pass_allowed(player) {
            self.passes = 0;
            self.bids[player.id()].remove(&current);
            if let Some((player, amount)) = self.only_bid(current) {
                self.current = PrivComId::values()
                    .find(|p| p.cost() > current.cost())
                    .cloned();
                Some((current, player, amount))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn pass_allowed(&self, player: &Player) -> bool {
        self.current_if_pass_allowed(player).is_some()
    }

    pub(crate) fn is_done(&self) -> bool {
        self.current.is_none()
    }

    pub(crate) fn in_auction(&self) -> bool {
        if let Some(current) = self.current {
            current.cost() != self.max_bid(current)
        } else {
            false
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
                    max_bid = *bids.get(&private).unwrap();
                }
            }
        }
        max_bid
    }

    fn only_bid(&self, private: PrivComId) -> Option<(usize, u32)> {
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
}
