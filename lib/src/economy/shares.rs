use crate::PubComId;
use std::collections::{HashMap, HashSet};

/// Represents any compination of shares
#[derive(Clone, Debug, Default)]
pub struct Shares {
    shares: HashMap<PubComId, u32>,
    president_shares: HashSet<PubComId>,
}

impl Shares {
    /// Returns an empty `Shares`
    pub fn new() -> Self {
        let mut shares = HashMap::new();
        for &id in PubComId::values() {
            shares.insert(id, 0);
        }
        Self {
            shares,
            president_shares: HashSet::new(),
        }
    }

    /// Returns an empty `Shares`
    pub fn with_all_shares() -> Self {
        let mut shares = HashMap::new();
        let mut president_shares = HashSet::new();
        for &id in PubComId::values() {
            shares.insert(id, 0);
            president_shares.insert(id);
        }
        Self {
            shares,
            president_shares,
        }
    }
}
