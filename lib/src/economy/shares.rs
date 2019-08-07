use crate::PubComId;
use std::collections::HashMap;

/// Represents any compination of shares
#[derive(Clone, Debug, Default)]
pub struct Shares {
    shares: HashMap<PubComId, u32>,
}

impl Shares {
    /// Returns an empty `Shares`
    pub fn new() -> Self {
        let mut shares = HashMap::new();
        for &id in PubComId::values() {
            shares.insert(id, 0);
        }
        Self { shares }
    }
}
