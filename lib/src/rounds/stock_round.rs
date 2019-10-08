use crate::PubComId;
use std::collections::HashSet;

/// Represents a stock round
#[derive(Clone, Debug)]
pub struct StockRound {
    sell_allowed: bool,
    pub_coms_sold: HashSet<(usize, PubComId)>,
}

impl StockRound {
    pub(crate) fn new(sell_allowed: bool) -> Self {
        StockRound {
            sell_allowed,
            pub_coms_sold: HashSet::new(),
        }
    }
}
