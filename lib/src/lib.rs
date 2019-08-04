#![warn(missing_docs)]
//! Game logic for the [18xx](https://boardgamegeek.com/boardgamefamily/19/18xx) board game [18Chesapeake](https://boardgamegeek.com/boardgame/253608/18chesapeake).

mod hex;
mod ids;
mod map;
mod rail;
mod stops;
mod tile;
mod tile_set;

pub use crate::hex::Hex;
pub use crate::ids::*;
pub use crate::map::Map;
pub use crate::rail::Rail;
pub use crate::stops::*;
pub use crate::tile::Tile;
pub use crate::tile_set::TileSet;

pub(crate) static INVALID_TOML: &str = "TOML is invalid";
