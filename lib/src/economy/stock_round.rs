use crate::PubComId;
use std::collections::HashSet;

/// Represents a stock round
#[derive(Clone, Debug)]
pub struct StockRound {
    sell_allowed: bool,
    last_actor: usize,
    player_count: usize,
    pub_coms_sold: HashSet<(usize, PubComId)>,
    passes: usize,
}

impl StockRound {
    pub(crate) fn new(sell_allowed: bool, last_actor: usize, player_count: usize) -> Self {
        StockRound {
            sell_allowed,
            last_actor,
            player_count,
            pub_coms_sold: HashSet::new(),
            passes: 0,
        }
    }

    /// Returns whether everyone has passed
    pub(crate) fn pass(&mut self) -> bool {
        self.passes += 1;
        if self.passes == self.player_count {
            self.passes = 0;
            true
        } else {
            false
        }
    }
}
