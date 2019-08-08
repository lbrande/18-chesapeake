use crate::{PubComId, TrainId};
use std::collections::HashSet;

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
