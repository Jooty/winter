use super::tile::*;
use rand::Rng;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        // TODO: Replace this with proper map generation.
        let tiles = vec![vec![Tile::new(Type::Dirt); height]; width];

        let mut map = Map { width, height, tiles };
        map.generate_trees(10);

        map
    }

    fn generate_trees(&mut self, percentage: i32) {
        let mut rng = rand::thread_rng();
        for i in self.tiles.iter_mut().flatten() { 
            let random_num = rng.gen_range(1..=100);
            if random_num <= percentage {
                continue;
            }

            println!("Place tree!");

            // If result is true, we want to place a tree on this tile.
            i.tile_type = Type::Tree(TreeType::Oak);
        }
    }
}
