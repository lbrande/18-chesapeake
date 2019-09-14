use crate::PrivComId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Represents the auction for privates
pub struct PrivateAuction {
    current: PrivComId,
    bids: Vec<HashMap<PrivComId, u32>>,
}

impl PrivateAuction {
    pub(crate) fn new(player_count: usize) -> Self {
        Self {
            current: PrivComId::DAndR,
            bids: vec![HashMap::new(); player_count],
        }
    }

    pub(crate) fn place_bid(
        &mut self,
        start_capital: u32,
        player: usize,
        private: PrivComId,
        amount: u32,
    ) -> bool {
        if self.bid_allowed(start_capital, player, private, amount) {
            self.bids[player].insert(private, amount);
            true
        } else {
            false
        }
    }

    fn bid_allowed(
        &self,
        start_capital: u32,
        player: usize,
        private: PrivComId,
        amount: u32,
    ) -> bool {
        let total_amount: u32 = self.bids[player].iter().map(|(_, a)| a).sum();
        amount + total_amount <= start_capital
            && amount + 5 >= self.max_bid(private)
            && (!self.bids[player].contains_key(&private)
                || (private == self.current && self.bids[player].get(&private).unwrap() == &0))
    }

    fn max_bid(&self, private: PrivComId) -> u32 {
        let mut highest_bid = private.get_cost();
        for bids in &self.bids {
            if let Some(&bid) = bids.get(&private) {
                if bid > highest_bid {
                    highest_bid = *bids.get(&private).unwrap();
                }
            }
        }
        highest_bid
    }
}
