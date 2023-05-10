pub struct Tile {
    tile_type: Type,
}

pub enum Type {
    Dirt, // Does nothing.
    Mud,  // Slows unit.
    Path, // Speeds unit.
    Water // Wall, essentially.
}

impl Tile {
    pub fn new(tile_type: Type) -> Tile {
        Tile {
            tile_type
        }
    }
}
