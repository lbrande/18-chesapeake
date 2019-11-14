use crate::economy::Player;
use crate::PubComId;
use std::collections::HashSet;

/// Represents a stock round
#[derive(Clone, Debug)]
pub struct StockRound {
    action_performed: bool,
    sell_allowed: bool,
    pub_coms_sold: HashSet<(PubComId, usize)>,
}

impl StockRound {
    pub(crate) fn new(sell_allowed: bool) -> Self {
        StockRound {
            action_performed: false,
            sell_allowed,
            pub_coms_sold: HashSet::new(),
        }
    }

    pub(crate) fn insert_pub_com_sold(&mut self, pub_com: PubComId, player: &Player) {
        self.pub_coms_sold.insert((pub_com, player.id()));
    }

    pub(crate) fn set_action_performed(&mut self) {
        self.action_performed = true;
    }

    pub(crate) fn unset_action_performed(&mut self) {
        self.action_performed = false;
    }

    /// Returns whether the current player has performed an action
    pub fn action_performed(&self) -> bool {
        self.action_performed
    }

    /// Returns whether selling shares is allowed in this `StockRound`
    pub fn sell_allowed(&self) -> bool {
        self.sell_allowed
    }
}
