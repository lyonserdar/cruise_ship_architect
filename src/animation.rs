use bevy::prelude::*;
#[derive(Component, Default)]
pub enum Facing {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl From<(f32, f32)> for Facing {
    fn from((x, y): (f32, f32)) -> Self {
        if x.abs() > y.abs() {
            if x > 0.0 {
                Self::Right
            } else {
                Self::Left
            }
        } else {
            if y > 0.0 {
                Self::Up
            } else {
                Self::Down
            }
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    pub idle_up: (usize, usize),
    pub idle_down: (usize, usize),
    pub idle_left: (usize, usize),
    pub idle_right: (usize, usize),
    pub walk_up: (usize, usize),
    pub walk_down: (usize, usize),
    pub walk_left: (usize, usize),
    pub walk_right: (usize, usize),
}

impl AnimationIndices {
    pub fn get(&self, state: &AnimationState) -> (usize, usize) {
        match state {
            AnimationState::IdleUp => self.idle_up,
            AnimationState::IdleDown => self.idle_down,
            AnimationState::IdleLeft => self.idle_left,
            AnimationState::IdleRight => self.idle_right,
            AnimationState::WalkUp => self.walk_up,
            AnimationState::WalkDown => self.walk_down,
            AnimationState::WalkLeft => self.walk_left,
            AnimationState::WalkRight => self.walk_right,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
pub enum AnimationState {
    IdleUp,
    #[default]
    IdleDown,
    IdleLeft,
    IdleRight,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
}

#[derive(Component, Default)]
pub struct AnimationStates {
    pub current_state: AnimationState,
    pub last_state: AnimationState,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut AnimationStates,
    )>,
) {
    for (indices, mut timer, mut sprite, mut animation_states) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let current_animation_state = animation_states.current_state;
            let last_animation_state = animation_states.last_state;
            let current_indices = indices.get(&current_animation_state);
            if current_animation_state != last_animation_state {
                match current_animation_state {
                    AnimationState::IdleUp => sprite.index = indices.idle_up.0,
                    AnimationState::IdleDown => sprite.index = indices.idle_down.0,
                    AnimationState::IdleLeft => sprite.index = indices.idle_left.0,
                    AnimationState::IdleRight => sprite.index = indices.idle_right.0,
                    AnimationState::WalkUp => sprite.index = indices.walk_up.0,
                    AnimationState::WalkDown => sprite.index = indices.walk_down.0,
                    AnimationState::WalkLeft => sprite.index = indices.walk_left.0,
                    AnimationState::WalkRight => sprite.index = indices.walk_right.0,
                }

                animation_states.last_state = current_animation_state;
            }
            sprite.index = if sprite.index == current_indices.1 {
                current_indices.0
            } else {
                sprite.index + 1
            };
        }
    }
}
