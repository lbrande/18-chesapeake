use crate::PubComId;
use std::collections::{HashMap, HashSet};

/// Represents any compination of shares
#[derive(Clone, Debug)]
pub struct Shares {
    shares: HashMap<PubComId, u32>,
    presidencies: HashSet<PubComId>,
}

impl Shares {
    pub(crate) fn ipo_shares() -> Self {
        let mut shares = HashMap::new();
        let mut presidencies = HashSet::new();
        for &id in PubComId::values() {
            shares.insert(id, 8);
            presidencies.insert(id);
        }
        Self {
            shares,
            presidencies,
        }
    }

    pub(crate) fn empty_shares() -> Self {
        let mut shares = HashMap::new();
        for &id in PubComId::values() {
            shares.insert(id, 0);
        }
        Self {
            shares,
            presidencies: HashSet::new(),
        }
    }

    pub(crate) fn add_shares(&mut self, pub_com: PubComId, count: u32) {
        self.shares.entry(pub_com).and_modify(|c| *c += count);
    }

    pub(crate) fn remove_shares(&mut self, pub_com: PubComId, count: u32) {
        self.shares.entry(pub_com).and_modify(|c| *c -= count);
    }

    pub(crate) fn add_presidency(&mut self, pub_com: PubComId) {
        self.presidencies.insert(pub_com);
    }

    pub(crate) fn remove_presidency(&mut self, pub_com: PubComId) {
        self.presidencies.remove(&pub_com);
    }

    /// Returns the number of shares of `pub_com` in this `Shares`
    pub fn count(&self, pub_com: PubComId) -> u32 {
        *self.shares.get(&pub_com).unwrap()
    }

    /// Returns whether the presidency of `pub_com` is in this `Shares`
    pub fn contains_presidency(&self, pub_com: PubComId) -> bool {
        self.presidencies.contains(&pub_com)
    }
}
