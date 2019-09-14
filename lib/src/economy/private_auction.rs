use crate::PrivComId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Represents the auction for privates
pub struct PrivateAuction {
    bids: Vec<HashMap<PrivComId, u32>>,
}

impl PrivateAuction {
    pub(crate) fn new(player_count: usize) -> Self {
        Self {
            bids: vec![HashMap::new(); player_count],
        }
    }
}
