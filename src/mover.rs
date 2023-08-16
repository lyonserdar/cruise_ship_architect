use crate::prelude::*;
use pathfinding::prelude::astar;

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
pub struct MoverPath(pub Vec<GridNode>);

#[derive(Component)]
pub struct MoverTarget(pub GridNode);
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
            target.x as f32 * TILE_SIZE as f32,
            target.y as f32 * TILE_SIZE as f32,
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
    mut query: Query<Entity, (Without<MoverTarget>, Without<MoverPath>)>,
) {
    for entity in query.iter_mut() {
        let target = GridNode::new(
            rand::thread_rng().gen_range(0..GRID_WIDTH as i32),
            rand::thread_rng().gen_range(0..GRID_HEIGHT as i32),
        );

        commands.entity(entity).insert(MoverTarget(target));
    }
}

pub fn generate_path_for_movers(
    mut commands: Commands,
    grid: Res<Grid>,
    mut query: Query<(Entity, &Transform, &MoverTarget), Without<MoverPath>>,
) {
    for (entity, transform, target) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;
        let start = GridNode::new((x / TILE_SIZE as f32) as i32, (y / TILE_SIZE as f32) as i32);
        let target = target.0;

        let result = astar(
            &start,
            |n| grid.get_neighbors(n),
            |n| n.distance(&target) as u32 / 3,
            |n| *n == target,
        );

        if result.is_some() {
            commands.entity(entity).insert(MoverPath(result.unwrap().0));
        } else {
            commands.entity(entity).remove::<MoverTarget>();
        }
    }
}
