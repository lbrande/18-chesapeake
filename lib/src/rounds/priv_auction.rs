use crate::economy::Player;
use crate::{Game, PrivComId};
use std::collections::HashMap;

#[derive(Debug)]
/// Represents the auction for private companies
pub struct PrivAuction {
    players: Vec<Player>,
    current_player: usize,
    priority_player: usize,
    current_priv: Option<PrivComId>,
    bids: Vec<HashMap<PrivComId, u32>>,
    passes: usize,
}

impl PrivAuction {
    pub(crate) fn new(players: Vec<Player>) -> Self {
        let player_count = players.len();
        PrivAuction {
            players,
            current_player: 0,
            priority_player: 0,
            current_priv: Some(PrivComId::DAndR(20)),
            bids: vec![HashMap::new(); player_count],
            passes: 0,
        }
    }

    pub(crate) fn place_bid(&mut self, private: PrivComId, amount: u32) {
        if self.bid_allowed(private, amount) {
            self.passes = 0;
            self.bids[self.current_player].insert(private, amount);
            self.advance_current_player();
        }
    }

    pub(crate) fn bid_allowed(&self, private: PrivComId, amount: u32) -> bool {
        if let Some(current_priv) = self.current_priv {
            self.can_afford_bid(private, amount)
                && amount + 5 >= self.max_bid(private)
                && ((private == current_priv
                    && self.bids[self.current_player]
                        .get(&private)
                        .map_or(false, |&a| a != self.max_bid(private)))
                    || (private != current_priv
                        && current_priv.cost() == self.max_bid(current_priv)
                        && !self.bids[self.current_player].contains_key(&private)))
        } else {
            false
        }
    }

    /// Returns whether the private auction is done
    pub(crate) fn buy_current(&mut self) -> bool {
        if let Some(current_priv) = self.current_if_buy_allowed() {
            self.passes = 0;
            self.current_priv = PrivComId::values()
                .find(|p| p.cost() > current_priv.cost())
                .cloned();
            self.players[self.current_player].buy_priv(current_priv, current_priv.cost());
            self.priority_player = (self.current_player + 1) % self.players.len();
            self.advance_current_player();
        }
        self.is_done()
    }

    pub(crate) fn buy_allowed(&self) -> bool {
        self.current_if_buy_allowed().is_some()
    }

    /// Returns whether the private auction is done
    pub(crate) fn pass(&mut self) -> bool {
        if self.in_auction() {
            self.pass_in_auction()
        } else {
            self.pass_on_current_priv();
            false
        }
    }

    pub(crate) fn pass_allowed(&self) -> bool {
        self.current_if_pass_allowed().is_some()
    }

    /// Returns the players and the priority player id
    pub(crate) fn state(self) -> (Vec<Player>, usize) {
        (self.players, self.priority_player)
    }

    fn is_done(&self) -> bool {
        self.current_priv.is_none()
    }

    pub(crate) fn in_auction(&self) -> bool {
        if let Some(current_priv) = self.current_priv {
            current_priv.cost() != self.max_bid(current_priv)
        } else {
            false
        }
    }

    pub(crate) fn next_player_in_auction(&self) -> Option<usize> {
        if let Some(current_priv) = self.current_priv {
            if current_priv.cost() == self.max_bid(current_priv) {
                None
            } else {
                self.next_player_to_bid(current_priv)
            }
        } else {
            None
        }
    }

    fn advance_current_player(&mut self) {
        if let Some(player_id) = self.next_player_in_auction() {
            self.current_player = player_id;
        } else {
            self.current_player = (self.current_player + 1) % self.players.len();
        }
    }

    fn next_player_to_bid(&self, private: PrivComId) -> Option<usize> {
        if let Some(player_id) = self.player_id_with_max_bid(private) {
            for i in 1..self.players.len() {
                if self.bids[(player_id + i) % self.players.len()]
                    .get(&private)
                    .is_some()
                {
                    return Some(i);
                }
            }
        }
        None
    }

    fn current_if_buy_allowed(&self) -> Option<PrivComId> {
        if let Some(current_priv) = self.current_priv {
            if current_priv.cost() == self.max_bid(current_priv)
                && self.can_afford_bid(current_priv, current_priv.cost())
            {
                Some(current_priv)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn pass_on_current_priv(&mut self) {
        if let Some(current_priv) = self.current_if_pass_allowed() {
            self.passes += 1;
            if self.passes == self.players.len() {
                self.passes = 0;
                if let PrivComId::DAndR(cost) = current_priv {
                    self.current_priv = Some(PrivComId::DAndR(cost - 5));
                } else {
                    Game::operate_priv_coms(&mut self.players);
                }
            }
            self.advance_current_player();
        }
    }

    /// Returns whether the private auction is done
    fn pass_in_auction(&mut self) -> bool {
        if let Some(current_priv) = self.current_if_pass_allowed() {
            self.passes = 0;
            self.bids[self.current_player].remove(&current_priv);
            if let Some((player_id, amount)) = self.only_bid(current_priv) {
                self.current_priv = PrivComId::values()
                    .find(|p| p.cost() > current_priv.cost())
                    .cloned();
                self.players[player_id].buy_priv(current_priv, amount);
            }
            self.advance_current_player();
        }
        self.is_done()
    }

    fn current_if_pass_allowed(&self) -> Option<PrivComId> {
        if let Some(current_priv) = self.current_priv {
            if current_priv.cost() != 0
                && (current_priv.cost() == self.max_bid(current_priv)
                    || self.bids[self.current_player].contains_key(&current_priv))
            {
                Some(current_priv)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn can_afford_bid(&self, private: PrivComId, amount: u32) -> bool {
        let previous_total_amount: u32 =
            self.bids[self.current_player].iter().map(|(_, a)| a).sum();
        if let Some(&bid) = self.bids[self.current_player].get(&private) {
            amount + previous_total_amount - bid <= self.players[self.current_player].capital()
        } else {
            amount + previous_total_amount <= self.players[self.current_player].capital()
        }
    }

    fn max_bid(&self, private: PrivComId) -> u32 {
        let mut max_bid = private.cost();
        for bids in &self.bids {
            if let Some(&bid) = bids.get(&private) {
                if bid > max_bid {
                    max_bid = bid;
                }
            }
        }
        max_bid
    }

    fn player_id_with_max_bid(&self, private: PrivComId) -> Option<usize> {
        let mut player_id = None;
        let mut max_bid = private.cost();
        for i in 0..self.players.len() {
            if let Some(&bid) = self.bids[i].get(&private) {
                if bid > max_bid {
                    player_id = Some(i);
                    max_bid = bid;
                }
            }
        }
        player_id
    }

    fn only_bid(&self, private: PrivComId) -> Option<(usize, u32)> {
        let mut only_bid = None;
        for i in 0..self.players.len() {
            if let Some(&bid) = self.bids[i].get(&private) {
                if only_bid.is_none() {
                    only_bid = Some((i, bid));
                } else {
                    return None;
                }
            }
        }
        only_bid
    }
}
