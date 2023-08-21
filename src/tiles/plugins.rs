use crate::prelude::*;
use crate::tiles::systems::add_spatial_for_tiles::add_spatial_for_tiles;
use crate::tiles::systems::remove_floor::remove_floor;
use crate::tiles::systems::spawn_floor::spawn_floor;
use crate::tiles::systems::spawn_wall::spawn_wall;
use crate::tiles::systems::update_floor_sprites::update_floor_sprites;
use crate::tiles::systems::update_tile_lookup::update_tile_lookup;
use crate::tiles::systems::update_wall_sprites::update_wall_sprites;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            add_spatial_for_tiles.run_if(in_state(GamePlayState::Playing)),
        )
        .add_systems(
            Update,
            (spawn_floor, spawn_wall).run_if(in_state(GamePlayState::Playing)),
        )
        .add_systems(
            Update,
            remove_floor.run_if(in_state(GamePlayState::Playing)),
        )
        .add_systems(
            Update,
            update_tile_lookup.run_if(in_state(GamePlayState::Playing)),
        )
        .add_systems(
            Update,
            (update_wall_sprites, update_floor_sprites).run_if(in_state(GamePlayState::Playing)),
        );
    }
}
