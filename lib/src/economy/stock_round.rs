use crate::PubComId;
use std::collections::HashSet;

/// Represents a stock round
#[derive(Clone, Debug)]
pub struct StockRound {
    pub_coms_sold: HashSet<(usize, PubComId)>,
    sell_allowed: bool,
}

impl StockRound {
    pub(crate) fn new(sell_allowed: bool) -> Self {
        StockRound {
            pub_coms_sold: HashSet::new(),
            sell_allowed,
        }
    }
}
