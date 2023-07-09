use std::thread;
use std::time::Duration;
use crate::map::Map;

mod parser;
mod map;

const TIMEOUT: Duration = Duration::from_millis(100);

fn main() {
    let input = include_str!("../inputs/input.txt");
    let (leftover, rows) = parser::parse(input).unwrap();
    assert!(leftover.is_empty());

    let mut map = Map::new(&rows);
    // println!("{}", map);

    while map.tick() {
        // println!("{}", map);
        // thread::sleep(TIMEOUT);
    }

    map.process_still_water();
    println!("{}", map);
    println!("All water tiles count: {}", map.count_all_water());
    println!("Still water tiles count: {}", map.count_still_water());
}
