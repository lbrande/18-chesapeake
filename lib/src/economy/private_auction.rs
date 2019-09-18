use crate::economy::Player;
use crate::PrivComId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Represents the auction for privates
pub struct PrivateAuction {
    current: Option<PrivComId>,
    bids: Vec<HashMap<PrivComId, u32>>,
    passes: usize,
}

impl PrivateAuction {
    pub(crate) fn new(player_count: usize) -> Self {
        Self {
            current: Some(PrivComId::DAndR(20)),
            bids: vec![HashMap::new(); player_count],
            passes: 0,
        }
    }

    // Returns whether the bid was placed
    pub(crate) fn place_bid(
        &mut self,
        player: &Player,
        private: PrivComId,
        amount: u32,
    ) -> bool {
        if self.bid_allowed(player, private, amount) {
            self.passes = 0;
            self.bids[player.id()].insert(private, amount);
            true
        } else {
            false
        }
    }

    /// Returns the private company bought if any
    pub(crate) fn buy_current(&mut self, player: &Player) -> Option<PrivComId> {
        if let Some(current) = self.current {
            if current.get_cost() == self.max_bid(current)
                && self.can_afford_bid(player, current, current.get_cost())
            {
                self.passes = 0;
                self.current = PrivComId::values()
                    .find(|p| p.get_cost() > current.get_cost())
                    .cloned();
                Some(current)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns whether the pass was done and whether everyone has passed
    pub(crate) fn pass_on_current(&mut self, player_count: usize) -> (bool, bool) {
        if let Some(current) = self.current {
            if current.get_cost() == self.max_bid(current) {
                if current.get_cost() == 0 {
                    (false, false)
                } else {
                    self.passes += 1;
                    if self.passes == player_count {
                        self.passes = 0;
                        if let PrivComId::DAndR(cost) = current {
                            self.current = Some(PrivComId::DAndR(cost - 5));
                        }
                        (true, true)
                    } else {
                        (true, false)
                    }
                }
            } else {
                (false, false)
            }
        } else {
            (false, false)
        }
    }

    /// Returns the private company sold, the player who bought it and the price it was bought for if a private company was sold
    pub(crate) fn pass_in_auction(&mut self, player: &Player) -> Option<(PrivComId, usize, u32)> {
        if let Some(current) = self.current {
            if self.bids[player.id()].contains_key(&current) {
                self.passes = 0;
                self.bids[player.id()].remove(&current);
                if let Some((player, amount)) = self.only_bid(current) {
                    self.current = PrivComId::values()
                        .find(|p| p.get_cost() > current.get_cost())
                        .cloned();
                    Some((current, player, amount))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bid_allowed(&self, player: &Player, private: PrivComId, amount: u32) -> bool {
        if let Some(current) = self.current {
            self.can_afford_bid(player, private, amount)
                && amount + 5 >= self.max_bid(private)
                && ((private == current
                    && self.bids[player.id()]
                        .get(&private)
                        .map_or(false, |&a| a != self.max_bid(private)))
                    || (private != current && !self.bids[player.id()].contains_key(&private)))
        } else {
            false
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
        let mut max_bid = private.get_cost();
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
