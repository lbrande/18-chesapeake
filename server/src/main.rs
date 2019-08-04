use lib::Map;
use lib::TileSet;
use std::fs::*;

fn main() {
    let tile_set = read_to_string("conf/tile_set.toml").unwrap();
    let tile_set = TileSet::from_toml(&tile_set);
    println!("{:#?}", tile_set);
    let map = read_to_string("conf/map.toml").unwrap();
    let map = Map::from_toml(&map);
    println!("{:#?}", map);
}
