use crate::{PubComId, TrainId};
use std::collections::HashSet;

#[derive(Clone, Debug)]
/// Represents a public company
pub struct PublicCompany {
    id: PubComId,
    capital: u32,
    trains: HashSet<TrainId>,
}
