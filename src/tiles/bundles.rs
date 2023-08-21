use crate::prelude::*;
use crate::tiles::components::tile::Tile;
use crate::tiles::components::walkable::Walkable;

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub walkable: Walkable,
    pub position: Position,
}

impl Default for TileBundle {
    fn default() -> Self {
        Self {
            tile: Tile,
            walkable: Walkable,
            position: Position::new(0, 0),
        }
    }
}
