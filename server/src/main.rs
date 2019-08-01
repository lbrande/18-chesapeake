use lib::Map;
use lib::TileSet;
use std::fs::*;

fn main() {
    let tile_set = read_to_string("tile_set.toml").unwrap();
    let tile_set = tile_set.parse::<TileSet>().unwrap();
    println!("{:#?}", tile_set);
    let map = read_to_string("map.toml").unwrap();
    let map = map.parse::<Map>().unwrap();
    println!("{:#?}", map);
}
