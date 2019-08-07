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
    /// Returns a `Player` with `capital` initial capital
    pub fn new(capital: u32) -> Self {
        Self {
            capital,
            shares: Shares::player_shares(),
            privates: HashSet::new(),
        }
    }
}
