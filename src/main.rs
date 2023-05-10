mod env;

use env::Map;

fn main() {
    println!("Hello, world!");

    let map = Map::new(20, 20);

    for i in map.tiles.iter().flatten() {
        println!(" {}", i);
    }
}
