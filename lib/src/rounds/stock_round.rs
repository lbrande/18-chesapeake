use crate::economy::Player;
use crate::PubComId;
use std::collections::HashSet;

/// Represents a stock round
#[derive(Clone, Debug)]
pub struct StockRound {
    players: Vec<Player>,
    current_player: usize,
    priority_player: usize,
    sell_allowed: bool,
    pub_coms_sold: HashSet<(usize, PubComId)>,
    passes: usize,
}

impl StockRound {
    pub(crate) fn new(players: Vec<Player>, priority_player: usize, sell_allowed: bool) -> Self {
        StockRound {
            players,
            current_player: priority_player,
            priority_player,
            sell_allowed,
            pub_coms_sold: HashSet::new(),
            passes: 0,
        }
    }

    pub(crate) fn sell_allowed(&mut self, pub_com: PubComId, count: u32) {}

    /// Returns whether everyone has passed
    pub(crate) fn pass(&mut self) -> bool {
        self.passes += 1;
        if self.passes == self.players.len() {
            self.passes = 0;
            true
        } else {
            false
        }
    }
}
