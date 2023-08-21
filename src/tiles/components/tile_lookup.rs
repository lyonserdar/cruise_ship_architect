use crate::prelude::*;
pub use bevy::utils::HashMap;

/// This will be loaded when the scene is created
#[derive(Component)]
pub struct TileLookup {
    pub tiles: HashMap<Position, Entity>,
}

impl TileLookup {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.tiles.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.tiles.values_mut()
    }
}

impl Default for TileLookup {
    fn default() -> Self {
        Self::new()
    }
}
