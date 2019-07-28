use lib::TileSet;
use std::fs::*;

fn main() {
    let tile_set = read_to_string("tile_set.toml").unwrap();
    let tile_set = TileSet::from_string(&tile_set);
    println!("Hello, world!");
}
