use crate::prelude::*;
use crate::tiles::components::in_tile_lookup::InTileLookup;
use crate::tiles::components::position::Position;
use crate::tiles::components::tile_lookup::TileLookup;

pub fn update_tile_lookup(
    mut commands: Commands,
    mut tile_lookup_query: Query<&mut TileLookup>,
    tile_query: Query<(Entity, &Position), Without<InTileLookup>>,
) {
    let mut tile_lookup = match tile_lookup_query.get_single_mut() {
        Ok(tile_lookup) => tile_lookup,
        Err(error) => {
            error!("No TileLookup Component Found: {}", error);
            return;
        }
    };

    for (entity, position) in tile_query.iter() {
        tile_lookup.tiles.insert(*position, entity);
        commands.entity(entity).insert(InTileLookup);
    }
}
