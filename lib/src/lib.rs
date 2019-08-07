#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
//! Game logic for the [18xx](https://boardgamegeek.com/boardgamefamily/19/18xx) board game
//! [18Chesapeake](https://boardgamegeek.com/boardgame/253608/18chesapeake).

pub mod economy;
pub mod geography;
mod ids;

pub use ids::*;

pub(crate) static INVALID_TOML: &str = "TOML is invalid";
