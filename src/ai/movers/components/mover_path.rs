use crate::tiles::Position;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MoverPath(pub Vec<Position>);
