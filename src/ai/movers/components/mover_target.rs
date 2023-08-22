use crate::tiles::Position;
use bevy::prelude::*;

#[derive(Component)]
pub struct MoverTarget(pub Position);
