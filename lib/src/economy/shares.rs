use crate::PubComId;
use std::collections::{HashMap, HashSet};

/// Represents any compination of shares
#[derive(Clone, Debug)]
pub struct Shares {
    shares: HashMap<PubComId, u32>,
    president_shares: HashSet<PubComId>,
    kind: Kind,
}

impl Shares {
    /// Returns a `Shares` for the IPO
    pub fn ipo_shares() -> Self {
        let mut shares = HashMap::new();
        let mut president_shares = HashSet::new();
        for &id in PubComId::values() {
            shares.insert(id, 0);
            president_shares.insert(id);
        }
        Self {
            shares,
            president_shares,
            kind: Kind::Ipo
        }
    }

    /// Returns a `Shares` for the bank pool
    pub fn bank_pool_shares() -> Self {
        Self::with_kind(Kind::BankPool)
    }

    /// Returns a `Shares` for a player
    pub fn player_shares() -> Self {
        Self::with_kind(Kind::Player)
    }

    fn with_kind(kind: Kind) -> Self {
        let mut shares = HashMap::new();
        for &id in PubComId::values() {
            shares.insert(id, 0);
        }
        Self {
            shares,
            president_shares: HashSet::new(),
            kind
        }
    }

}

#[derive(Clone, Debug)]
enum Kind {
    Ipo,
    BankPool,
    Player,
}