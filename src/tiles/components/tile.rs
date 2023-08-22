use bevy::prelude::*;
pub use bevy::utils::HashMap;

// #[derive(Debug, Default)]
// pub struct Neighbors<T> {
//     pub east: Option<T>,
//     pub north_east: Option<T>,
//     pub north: Option<T>,
//     pub north_west: Option<T>,
//     pub west: Option<T>,
//     pub south_west: Option<T>,
//     pub south: Option<T>,
//     pub south_east: Option<T>,
// }

// This is just a tag component for now but it might have more use later
// like neighbors
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tile;
