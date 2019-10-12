use crate::economy::{ParTrack, Player, PubCom, Shares, StockChart};
use crate::geography::{Map, TileSet};
use crate::rounds::{PrivAuction, StockRound};
use crate::{PhaseId, PrivComId, PubComId, RoundId, TrainSet};
use std::collections::HashMap;
use std::fs;

static ACTION_FORBIDDEN: &str = "action is forbidden";

/// Represents a game
#[derive(Debug)]
pub struct Game {
    round: RoundId,
    phase: PhaseId,
    players: Vec<Player>,
    current_player: usize,
    priority_player: usize,
    passes: usize,
    pub_coms: HashMap<PubComId, PubCom>,
    map: Map,
    tile_set: TileSet,
    train_set: TrainSet,
    stock_chart: StockChart,
    par_track: ParTrack,
    ipo: Shares,
    bank_pool: Shares,
    bank_amount: u32,
}

impl Game {
    /// Returns a game
    pub fn new(player_count: usize) -> Self {
        if player_count < 2 || player_count > 6 {
            panic!("player_count out of bounds");
        }
        let mut players = Vec::with_capacity(player_count);
        for id in 0..player_count {
            players.push(Player::new(id, 2400 / player_count as u32));
        }
        Self {
            round: RoundId::PrivAuction(PrivAuction::new(player_count)),
            phase: PhaseId::Phase2,
            players,
            current_player: 0,
            priority_player: 0,
            passes: 0,
            pub_coms: HashMap::new(),
            map: Map::from_toml(&read_toml_file("map")),
            tile_set: TileSet::from_toml(&read_toml_file("tile_set")),
            train_set: TrainSet::from_toml(&read_toml_file("train_set")),
            stock_chart: StockChart::from_toml(&read_toml_file("stock_chart")),
            par_track: ParTrack::from_toml(&read_toml_file("par_track")),
            ipo: Shares::ipo_shares(),
            bank_pool: Shares::empty_shares(),
            bank_amount: 8000,
        }
    }

    fn enter_first_stock_round(&mut self) {
        self.round = RoundId::StockRound(StockRound::new(false));
        self.current_player = self.priority_player;
    }

    fn operate_priv_coms(players: &mut Vec<Player>) {
        for player in players {
            player.operate_priv_coms();
        }
    }

    fn advance_current_player(&mut self) {
        match &mut self.round {
            RoundId::StockRound(_) => {
                self.current_player = (self.current_player + 1) % self.players.len();
            }
            RoundId::PrivAuction(priv_auction) => {
                if let Some(player) = priv_auction.next_player_in_auction() {
                    self.current_player = player;
                } else {
                    self.current_player = (self.current_player + 1) % self.players.len();
                }
            }
            _ => (),
        }
    }
}

/// Returns whether placing a bid on a private company is allowed
pub fn bid_priv_allowed(game: &Game, private: PrivComId, amount: u32) -> bool {
    if let RoundId::PrivAuction(priv_auction) = &game.round {
        let current_player = &game.players[game.current_player];
        if let Some(current_priv) = priv_auction.current_priv() {
            priv_auction.can_afford_bid(&current_player, private, amount)
                && amount + 5 >= priv_auction.max_bid(private)
                && ((private == current_priv
                    && priv_auction
                        .bids(&current_player)
                        .get(&private)
                        .map_or(false, |&a| a != priv_auction.max_bid(private)))
                    || (private != current_priv
                        && current_priv.cost() == priv_auction.max_bid(current_priv)
                        && !priv_auction
                            .bids(&game.players[game.current_player])
                            .contains_key(&private)))
        } else {
            false
        }
    } else {
        false
    }
}

/// Places a bid on a private company
pub fn bid_priv(game: &mut Game, private: PrivComId, amount: u32) {
    if !bid_priv_allowed(game, private, amount) {
        panic!(ACTION_FORBIDDEN);
    }
    if let RoundId::PrivAuction(priv_auction) = &mut game.round {
        game.passes = 0;
        priv_auction.insert_bid(&game.players[game.current_player], private, amount);
        priv_auction.zero_non_max_bids(private);
    }
    game.advance_current_player();
}

/// Returns whether buying a private company is allowed
pub fn buy_priv_allowed(game: &Game) -> bool {
    if let RoundId::PrivAuction(priv_auction) = &game.round {
        priv_auction
            .current_if_buy_allowed(&game.players[game.current_player])
            .is_some()
    } else {
        false
    }
}

/// Buys a private company
pub fn buy_priv(game: &mut Game) {
    if !buy_priv_allowed(game) {
        panic!(ACTION_FORBIDDEN);
    }
    if let RoundId::PrivAuction(priv_auction) = &mut game.round {
        let current_player = &mut game.players[game.current_player];
        if let Some(current_priv) = priv_auction.current_if_buy_allowed(&current_player) {
            game.passes = 0;
            priv_auction.advance_current_priv();
            current_player.buy_priv(current_priv, current_priv.cost());
            game.priority_player = (game.current_player + 1) % game.players.len();
            if priv_auction.done() {
                game.enter_first_stock_round();
                return;
            }
        }
    }
    game.advance_current_player();
}

/// Returns whether passing is allowed
pub fn pass_allowed(game: &Game) -> bool {
    match &game.round {
        RoundId::StockRound(_) => true,
        RoundId::PrivAuction(priv_auction) => priv_auction
            .current_if_pass_allowed(&game.players[game.current_player])
            .is_some(),
        _ => false,
    }
}

/// Passes
pub fn pass(game: &mut Game) {
    if !pass_allowed(game) {
        panic!(ACTION_FORBIDDEN);
    }
    match &mut game.round {
        RoundId::StockRound(_) => {
            game.passes += 1;
            if game.passes == game.players.len() {
                game.passes = 0;
                // TODO
            }
        }
        RoundId::PrivAuction(priv_auction) => {
            let current_player = &game.players[game.current_player];
            if let Some(current_priv) = priv_auction.current_if_pass_allowed(&current_player) {
                if priv_auction.in_auction() {
                    game.passes = 0;
                    priv_auction.remove_bid(&current_player, current_priv);
                    if let Some((player, amount)) = priv_auction.only_bid(current_priv) {
                        priv_auction.advance_current_priv();
                        game.players[player].buy_priv(current_priv, amount);
                        if priv_auction.done() {
                            game.enter_first_stock_round();
                            return;
                        }
                    }
                } else {
                    game.passes += 1;
                    if game.passes == game.players.len() {
                        game.passes = 0;
                        if let PrivComId::DAndR(cost) = current_priv {
                            priv_auction.reduce_d_and_r_price(cost);
                        } else {
                            Game::operate_priv_coms(&mut game.players);
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }
    game.advance_current_player();
}

/// Returns whether selling shares is allowed
pub fn sell_shares_allowed(game: &Game, pub_com: PubComId, count: u32) -> bool {
    if let RoundId::StockRound(stock_round) = &game.round {
        let current_player = &game.players[game.current_player];
        let owned_count = current_player.shares().count(pub_com);
        let president = current_player.shares().president(pub_com);
        (!president
            || owned_count - count >= 2
            || game
                .players
                .iter()
                .any(|p| p.id() != current_player.id() && p.shares().count(pub_com) >= 2))
            && count + game.bank_pool.count(pub_com) <= 5
            && owned_count >= count
            && stock_round.sell_allowed()
    } else {
        false
    }
}

fn read_toml_file(name: &str) -> String {
    fs::read_to_string(format!("conf/{}.toml", name)).unwrap()
}
