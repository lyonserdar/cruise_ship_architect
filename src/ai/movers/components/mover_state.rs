use bevy::prelude::*;

#[derive(Component, Default)]
pub enum MoverState {
    #[default]
    Idle,
    Moving,
}
