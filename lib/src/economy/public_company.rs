use crate::{PubComId, TrainId};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
/// Represents a public company
pub struct PublicCompany {
    id: PubComId,
    capital: u32,
    trains: HashSet<TrainId>,
}

impl PublicCompany {
    pub(crate) fn new(id: PubComId, capital: u32) -> Self {
        Self {
            id,
            capital,
            trains: HashSet::new(),
        }
    }
}

impl PartialEq for PublicCompany {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PublicCompany {}

impl Hash for PublicCompany {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
