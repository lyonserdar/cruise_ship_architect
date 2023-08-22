use crate::ai::movers::components::mover_path::MoverPath;
use crate::ai::movers::components::mover_target::MoverTarget;
use crate::ai::movers::{Mover, MoverState};
use crate::animation::{AnimationState, AnimationStates, Facing};
use crate::constants::TILE_SIZE;
use bevy::prelude::*;

pub fn move_movers_with_target(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &Mover,
            &mut MoverPath,
            &mut MoverState,
            &mut Facing,
            &mut AnimationStates,
        ),
        With<MoverTarget>,
    >,
    time: Res<Time>,
) {
    for (entity, mut transform, mover, mut path, mut state, mut facing, mut animation_states) in
        query.iter_mut()
    {
        *state = MoverState::Moving;
        let target = path.0.first().unwrap();

        let destination = Vec3::new(
            target.0.x as f32 * TILE_SIZE as f32,
            target.0.y as f32 * TILE_SIZE as f32,
            0.0,
        );

        let distance = destination - transform.translation;
        let direction = distance.normalize();
        *facing = Facing::from((direction.x, direction.y));

        let velocity = direction * mover.speed * time.delta_seconds();

        let new_position = transform.translation + velocity;

        if new_position.distance(destination) < velocity.length() {
            transform.translation = destination;
            // mover.path.remove(0);
            path.0.remove(0);

            // if mover.path.is_empty() {
            if path.0.is_empty() {
                // mover.target = None;
                *state = MoverState::Idle;
                *facing = Facing::Down;
                commands.entity(entity).remove::<MoverTarget>();
                commands.entity(entity).remove::<MoverPath>();
                animation_states.current_state = AnimationState::default();
            }
        } else {
            transform.translation = new_position;
            match *facing {
                Facing::Up => {
                    animation_states.current_state = AnimationState::WalkUp;
                }
                Facing::Down => {
                    animation_states.current_state = AnimationState::WalkDown;
                }
                Facing::Left => {
                    animation_states.current_state = AnimationState::WalkLeft;
                }
                Facing::Right => {
                    animation_states.current_state = AnimationState::WalkRight;
                }
            }
        }

        // TODO: layer
        transform.translation.z = 0.1;
    }
}
