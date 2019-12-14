use crate::economy::Player;
use crate::{Game, PubComId};
use std::collections::HashSet;

/// Represents a stock round
#[derive(Clone, Debug)]
pub struct StockRound {
    action_performed: bool,
    sell_allowed: bool,
    pub_coms_sold: HashSet<(PubComId, usize)>,
    player_count: usize,
    current_player: usize,
    passes: usize,
}

impl StockRound {
    pub(crate) fn new(sell_allowed: bool, player_count: usize, priority_player: usize) -> Self {
        StockRound {
            action_performed: false,
            sell_allowed,
            pub_coms_sold: HashSet::new(),
            player_count,
            current_player: priority_player,
            passes: 0,
        }
    }

    pub(crate) fn pass_allowed(&self, certificate_count: u32) -> bool {
        !self.action_performed() && certificate_count <= Game::certificate_limit(self.player_count)
    }

    pub(crate) fn pass(&mut self) -> bool {
        self.passes += 1;
        if self.passes == self.player_count {
            true
        } else {
            false
        }
    }

    pub(crate) fn buy_ipo_share_allowed(
        &self,
        player: &Player,
        ipo_count: u32,
        certificate_count: u32,
        par: u32,
        pub_com: PubComId,
    ) -> bool {
        ipo_count > 0
            && player.shares().count(pub_com) < 6
            && certificate_count < Game::certificate_limit(self.player_count)
            && player.capital() >= par
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

    pub(crate) fn current_player(&self) -> usize {
        self.current_player
    }

    /// Returns whether the current player has performed an action
    pub fn action_performed(&self) -> bool {
        self.action_performed
    }

    /// Returns whether selling shares is allowed in this `StockRound`
    pub fn sell_allowed(&self) -> bool {
        self.sell_allowed
    }

    fn advance_current_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.player_count;
    }
}
