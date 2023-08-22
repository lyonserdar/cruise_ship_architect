use crate::tiles::components::position::Position;
use crate::tiles::components::tile::Tile;
use bevy::prelude::*;

pub fn add_spatial_for_tiles(
    mut commands: Commands,
    query: Query<
        (Entity, &Position),
        (
            With<Tile>,
            Without<Visibility>,
            Without<ComputedVisibility>,
            Without<Transform>,
            Without<GlobalTransform>,
        ),
    >,
) {
    for (entity, position) in query.iter() {
        commands.entity(entity).insert(SpatialBundle {
            transform: Transform::from_translation(position.to_world()),
            ..default()
        });
    }
}
