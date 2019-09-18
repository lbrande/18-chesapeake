use crate::economy::Shares;
use crate::PrivComId;
use std::collections::HashSet;

#[derive(Clone, Debug)]
/// Represents a player in a game
pub struct Player {
    id: usize,
    capital: u32,
    shares: Shares,
    priv_coms: HashSet<PrivComId>,
}

impl Player {
    pub(crate) fn new(id: usize, capital: u32) -> Self {
        Self {
            id,
            capital,
            shares: Shares::player_shares(),
            priv_coms: HashSet::new(),
        }
    }

    pub(crate) fn buy_private(&mut self, private: PrivComId, price: u32) {
        self.capital -= price;
        self.priv_coms.insert(private);
    }

    /// Returns the id of this player
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the capital of this player
    pub fn capital(&self) -> u32 {
        self.capital
    }
}
