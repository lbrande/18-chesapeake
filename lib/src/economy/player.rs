use crate::economy::Shares;
use crate::PrivComId;
use std::collections::HashSet;

#[derive(Clone, Debug)]
/// Represents a player in a game
pub struct Player {
    capital: u32,
    shares: Shares,
    privates: HashSet<PrivComId>,
}

impl Player {
    pub(crate) fn new(capital: u32) -> Self {
        Self {
            capital,
            shares: Shares::player_shares(),
            privates: HashSet::new(),
        }
    }

    pub(crate) fn buy_private(&mut self, private: PrivComId, price: u32) {
        self.capital -= price;
        self.privates.insert(private);
    }

    /// Returns the capital of this player
    pub fn capital(&self) -> u32 {
        self.capital
    }
}
