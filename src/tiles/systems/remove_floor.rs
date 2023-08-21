use crate::prelude::*;
use crate::tiles::components::floor::Floor;
use crate::tiles::components::floor_sprite::FloorSprite;
use crate::tiles::components::position::Position;
use crate::tiles::components::tile::Tile;
use crate::tiles::components::tile_lookup::TileLookup;

pub fn remove_floor(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    cursor_world_position: Res<CursorWorldPosition>,
    tile_lookup: Query<&TileLookup>,
    floor_query: Query<&Children, (With<Tile>, With<Floor>)>,
    query: Query<Entity, (With<Tile>, Or<(With<Wall>, With<Object>, With<Item>)>)>,
    sprite_query: Query<Entity, With<FloorSprite>>,
    build_mode_type: Res<BuildModeType>,
) {
    if *build_mode_type != BuildModeType::RemoveFloor {
        return;
    }

    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    if let Ok(tile_lookup) = tile_lookup.get_single() {
        let entity: Entity;
        let Vec2 { x, y } = cursor_world_position.position;
        let position = Position::new(x as i32, y as i32);

        if tile_lookup.tiles.contains_key(&position) {
            entity = *tile_lookup.tiles.get(&position).unwrap();

            if floor_query.get(entity).is_err() {
                println!("Floor does not exist");
                return;
            }

            if query.get(entity).is_ok() {
                println!("Remove the wall, object, or item first");
                return;
            }

            commands.entity(entity).remove::<Floor>();

            // Remove the child sprite
            // TODO: Find a better way to do this
            for &child in floor_query.get(entity).unwrap().iter() {
                if sprite_query.get(child).is_ok() {
                    commands.entity(child).despawn_recursive();
                    commands.entity(entity).remove_children(&[child]);
                }
            }
        }
    }
}
