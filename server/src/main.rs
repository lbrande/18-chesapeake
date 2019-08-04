use lib::Map;
use lib::ParTrack;
use lib::StockChart;
use lib::TileSet;
use std::fs::*;

fn main() {
    let tile_set = read_to_string("conf/tile_set.toml").unwrap();
    let tile_set = TileSet::from_toml(&tile_set);
    println!("{:#?}", tile_set);
    let map = read_to_string("conf/map.toml").unwrap();
    let map = Map::from_toml(&map);
    println!("{:#?}", map);
    let stock_chart = read_to_string("conf/stock_chart.toml").unwrap();
    let stock_chart = StockChart::from_toml(&stock_chart);
    println!("{:#?}", stock_chart);
    let par_track = read_to_string("conf/par_track.toml").unwrap();
    let par_track = ParTrack::from_toml(&par_track);
    println!("{:#?}", par_track);
}
