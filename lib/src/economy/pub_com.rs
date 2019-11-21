use crate::PrivComId;
use crate::TrainId;
use std::collections::HashSet;

#[derive(Clone, Debug)]
/// Represents a public company
pub struct PubCom {
    capital: u32,
    trains: HashSet<TrainId>,
    priv_coms: HashSet<PrivComId>,
    operated: bool,
}

impl PubCom {
    pub(crate) fn new(capital: u32) -> Self {
        PubCom {
            capital,
            trains: HashSet::new(),
            priv_coms: HashSet::new(),
            operated: false,
        }
    }
}
