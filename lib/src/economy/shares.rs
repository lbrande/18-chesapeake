use crate::PubComId;
use std::collections::{HashMap, HashSet};

/// Represents any compination of shares
#[derive(Clone, Debug)]
pub struct Shares {
    shares: HashMap<PubComId, u32>,
    president_shares: HashSet<PubComId>,
}

impl Shares {
    pub(crate) fn ipo_shares() -> Self {
        let mut shares = HashMap::new();
        let mut president_shares = HashSet::new();
        for &id in PubComId::values() {
            shares.insert(id, 8);
            president_shares.insert(id);
        }
        Self {
            shares,
            president_shares,
        }
    }

    pub(crate) fn empty_shares() -> Self {
        let mut shares = HashMap::new();
        for &id in PubComId::values() {
            shares.insert(id, 0);
        }
        Self {
            shares,
            president_shares: HashSet::new(),
        }
    }

    /// Returns the number of shares of `pub_com` in this `Shares`.
    pub fn count(&self, pub_com: PubComId) -> u32 {
        *self.shares.get(&pub_com).unwrap()
    }

    /// Returns whether the president share of `pub_com` is in this `Shares`.
    pub fn president(&self, pub_com: PubComId) -> bool {
        self.president_shares.contains(&pub_com)
    }
}
