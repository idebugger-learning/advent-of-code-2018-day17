use std::thread;
use std::time::Duration;
use crate::map::Map;

mod parser;
mod map;

const TIMEOUT: Duration = Duration::from_millis(50);

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

    println!("{}", map);
    println!("Water tiles count: {}", map.count_water());
}
