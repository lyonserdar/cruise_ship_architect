use crate::prelude::*;
use crate::tiles::components::floor::Floor;
use crate::tiles::components::position::Position;
use crate::tiles::components::tile::Tile;
use crate::tiles::components::tile_lookup::TileLookup;
use crate::tiles::components::walkable::Walkable;
use crate::tiles::components::wall::Wall;

pub fn spawn_wall(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    cursor_world_position: Res<CursorWorldPosition>,
    tile_lookup: Query<&TileLookup>,
    wall_query: Query<Entity, (With<Tile>, With<Wall>)>,
    floor_query: Query<Entity, (With<Tile>, With<Floor>)>,
    build_mode_type: Res<BuildModeType>,
) {
    if *build_mode_type != BuildModeType::Wall {
        return;
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(tile_lookup) = tile_lookup.get_single() {
            let entity: Entity;
            let Vec2 { x, y } = cursor_world_position.position;
            let position = Position::new(x as i32, y as i32);

            if tile_lookup.tiles.contains_key(&position) {
                entity = *tile_lookup.tiles.get(&position).unwrap();

                if wall_query.get(entity).is_ok() {
                    // Wall already exists in this tile
                    println!("Wall already exists in this tile");
                    return;
                }
                if floor_query.get(entity).is_err() {
                    // If it doesn't include Floor, don't add Wall
                    println!("Wall can't be placed on this tile, no floor.");
                    return;
                }

                commands.entity(entity).remove::<Walkable>().insert(Wall);
            } else {
                println!("Wall can't be placed on this tile, no floor. Also tile not in lookup.");
            }
        }
    }
}
