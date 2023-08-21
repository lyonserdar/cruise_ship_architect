use crate::prelude::*;

#[derive(Component)]
pub struct Mover {
    pub speed: f32,
}

impl Mover {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}
