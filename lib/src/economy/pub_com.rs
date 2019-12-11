use crate::{PrivComId, PubComId, TrainId};
use std::collections::HashSet;

#[derive(Clone, Debug)]
/// Represents a public company
pub struct PubCom {
    id: PubComId,
    capital: u32,
    stations_left: u32,
    trains: Vec<TrainId>,
    priv_coms: HashSet<PrivComId>,
    operated: bool,
}

impl PubCom {
    pub(crate) fn new(id: PubComId, capital: u32) -> Self {
        PubCom {
            id,
            capital,
            stations_left: id.station_count(),
            trains: Vec::new(),
            priv_coms: HashSet::new(),
            operated: false,
        }
    }
}
