use super::tile::Tile;

pub struct Map {
    width: f32,
    height: f32,
    tiles: Vec<Vec<Tile>>
}

impl Map {
    pub fn new(width: f32, height: f32) -> Map {
        // TODO: Replace this with proper map generation.
        let tiles = vec![vec![]];

        Map { width, height, tiles }
    }

    pub fn generate_trees(self, percentage: f32) {

    }
}
