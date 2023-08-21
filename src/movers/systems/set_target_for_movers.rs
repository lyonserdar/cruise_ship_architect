use crate::movers::components::mover::Mover;
use crate::movers::components::mover_path::MoverPath;
use crate::movers::components::mover_target::MoverTarget;
use crate::prelude::*;
use rand::seq::IteratorRandom;

pub fn set_target_for_movers(
    mut commands: Commands,
    mut query: Query<Entity, (With<Mover>, Without<MoverTarget>, Without<MoverPath>)>,
    position_query: Query<&Position, (With<Tile>, With<Floor>, With<Walkable>)>,
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
