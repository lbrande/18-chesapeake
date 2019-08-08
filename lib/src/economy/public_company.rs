use crate::TrainId;
use std::collections::HashSet;

#[derive(Clone, Debug)]
/// Represents a public company
pub struct PublicCompany {
    capital: u32,
    trains: HashSet<TrainId>,
}

impl PublicCompany {
    pub(crate) fn new(capital: u32) -> Self {
        Self {
            capital,
            trains: HashSet::new(),
        }
    }
}
