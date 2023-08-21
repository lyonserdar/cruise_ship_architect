use crate::movers::components::mover_path::MoverPath;
use crate::movers::components::mover_target::MoverTarget;
use crate::prelude::*;
use pathfinding::prelude::astar;

pub fn generate_path_for_movers(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &MoverTarget), Without<MoverPath>>,
    tile_query: Query<(Entity, &Position, &Walkable), (With<Tile>, With<Floor>)>,
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
