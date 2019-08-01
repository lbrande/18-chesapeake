use lib::TileSet;
use std::fs::*;

fn main() {
    let tile_set = read_to_string("tile_set.toml").unwrap();
    let tile_set = tile_set.parse::<TileSet>().unwrap();
    println!("{:?}", tile_set);
}
