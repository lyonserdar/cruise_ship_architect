use pathfinding::prelude::astar;
use rand::seq::IteratorRandom;

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

#[derive(Component, Default)]
pub struct MoverPath(pub Vec<Position>);

#[derive(Component)]
pub struct MoverTarget(pub Position);

#[derive(Component, Default)]
pub enum MoverState {
    #[default]
    Idle,
    Moving,
}

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

pub fn set_target_for_movers(
    mut commands: Commands,
    mut query: Query<Entity, (With<Mover>, Without<MoverTarget>, Without<MoverPath>)>,
    position_query: Query<&Position, (With<Tile>, With<Walkable>)>,
) {
    let mut rng = rand::thread_rng();
    for entity in query.iter_mut() {
        match position_query.iter().choose(&mut rng) {
            None => {
                // println!("No position found");
                // continue;
            }
            Some(position) => {
                commands.entity(entity).insert(MoverTarget(*position));
            }
        };
    }
}

pub fn generate_path_for_movers(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &MoverTarget), Without<MoverPath>>,
    tile_query: Query<(Entity, &Position, &Walkable), With<Tile>>,
    tile_lookup: Query<&TileLookup>,
) {
    for (entity, transform, target) in query.iter_mut() {
        let start = Position::from(transform.translation);
        let target = target.0;

        let result = astar(
            &start,
            |n| n.get_neighbors_and_costs(&tile_query, &tile_lookup),
            |n| n.distance(&target) as u32 / 3,
            |n| *n == target,
        );

        match result {
            None => {
                info!("No path found");
                // This target is not reachable, remove the target
                // TODO? Add a Unreachable component to the entity
                commands.entity(entity).remove::<MoverTarget>();
            }
            Some(result) => {
                let path = result.0;
                commands.entity(entity).insert(MoverPath(path));
            }
        }
    }
}
