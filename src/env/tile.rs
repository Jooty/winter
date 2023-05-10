use std::fmt;

pub struct Tile {
    pub tile_type: Type,
}

#[allow(dead_code)]
pub enum Type {
    Dirt,
    Mud,
    Path,
    Water,
    Tree(TreeType)
}

pub enum TreeType {
    Oak,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.tile_type {
            Type::Dirt => write!(f, "."),
            Type::Mud => write!(f, ","),
            Type::Path => write!(f, "x"),
            Type::Water => write!(f, "o"),
            Type::Tree(_) => write!(f, "T"),
        }
    }
}

impl Tile {
    pub fn new(tile_type: Type) -> Tile {
        Tile {
            tile_type
        }
    }
}
