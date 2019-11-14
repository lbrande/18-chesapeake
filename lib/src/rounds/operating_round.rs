use crate::PubComId;
use std::collections::HashSet;

/// Represents an operating round
#[derive(Clone, Debug)]
pub struct OperatingRound {
    operating_rounds_left: u32,
    pub_coms_to_operate: HashSet<PubComId>,
}

impl OperatingRound {
    pub(crate) fn new(operating_rounds_left: u32) -> Self {
        OperatingRound {
            operating_rounds_left,
            pub_coms_to_operate: PubComId::values().copied().collect()
        }
    }
}
