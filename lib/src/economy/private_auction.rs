use crate::PrivComId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Represents the auction for privates
pub struct PrivateAuction {
    current: Option<PrivComId>,
    bids: Vec<HashMap<PrivComId, u32>>,
    passes: u32,
}

impl PrivateAuction {
    pub(crate) fn new(player_count: usize) -> Self {
        Self {
            current: Some(PrivComId::DAndR),
            bids: vec![HashMap::new(); player_count],
            passes: 0,
        }
    }

    pub(crate) fn place_bid(
        &mut self,
        capital: u32,
        player: usize,
        private: PrivComId,
        amount: u32,
    ) -> bool {
        if self.bid_allowed(capital, player, private, amount) {
            self.bids[player].insert(private, amount);
            self.passes = 0;
            true
        } else {
            false
        }
    }

    pub(crate) fn buy_current(&mut self, capital: u32, player: usize) -> bool {
        if let Some(current) = self.current {
            if current.get_cost() == self.max_bid(current)
                && self.can_afford_bid(capital, player, current, current.get_cost())
            {
                self.current = PrivComId::values()
                    .find(|p| p.get_cost() > current.get_cost())
                    .cloned();
                self.passes = 0;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub(crate) fn pass(&mut self, player: usize) -> u32 {
        if let Some(current) = self.current {
            if current.get_cost() == self.max_bid(current) {
                self.passes += 1;
            } else {
                self.passes = 0;
                self.bids[player].remove(&current);
            }
        } else {
            self.passes = 0;
        }
        self.passes
    }

    fn bid_allowed(&self, capital: u32, player: usize, private: PrivComId, amount: u32) -> bool {
        if let Some(current) = self.current {
            self.can_afford_bid(capital, player, private, amount)
                && amount + 5 >= self.max_bid(private)
                && ((private == current
                    && self.bids[player]
                        .get(&private)
                        .map_or(false, |&a| a != self.max_bid(private)))
                    || (private != current && !self.bids[player].contains_key(&private)))
        } else {
            false
        }
    }

    fn can_afford_bid(&self, capital: u32, player: usize, private: PrivComId, amount: u32) -> bool {
        let previous_total_amount: u32 = self.bids[player].iter().map(|(_, a)| a).sum();
        if let Some(&bid) = self.bids[player].get(&private) {
            amount + previous_total_amount - bid <= capital
        } else {
            amount + previous_total_amount <= capital
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
}
