use lib::economy::{ParTrack, StockChart};
use lib::geography::{Map, TileSet};
use lib::TrainSet;
use std::fs::read_to_string;

fn main() {
    let map = read_to_string("conf/map.toml").unwrap();
    let map = Map::from_toml(&map);
    println!("{:#?}", map);

    let par_track = read_to_string("conf/par_track.toml").unwrap();
    let par_track = ParTrack::from_toml(&par_track);
    println!("{:#?}", par_track);

    let stock_chart = read_to_string("conf/stock_chart.toml").unwrap();
    let stock_chart = StockChart::from_toml(&stock_chart);
    println!("{:#?}", stock_chart);

    let tile_set = read_to_string("conf/tile_set.toml").unwrap();
    let tile_set = TileSet::from_toml(&tile_set);
    println!("{:#?}", tile_set);

    let train_set = read_to_string("conf/train_set.toml").unwrap();
    let train_set = TrainSet::from_toml(&train_set);
    println!("{:#?}", train_set);
}
