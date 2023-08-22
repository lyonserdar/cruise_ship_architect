use crate::cursor::CursorWorldPosition;
use crate::game_ui::BuildModeType;
use crate::tiles::components::position::Position;
use crate::tiles::components::tile::Tile;
use crate::tiles::components::tile_lookup::TileLookup;
use crate::tiles::components::wall_sprite::WallSprite;
use crate::tiles::{Item, Object, Walkable, Wall};
use bevy::prelude::*;

pub fn remove_wall(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    cursor_world_position: Res<CursorWorldPosition>,
    tile_lookup: Query<&TileLookup>,
    wall_query: Query<&Children, (With<Tile>, With<Wall>)>,
    query: Query<Entity, (With<Tile>, Or<(With<Object>, With<Item>)>)>,
    sprite_query: Query<Entity, With<WallSprite>>,
    build_mode_type: Res<BuildModeType>,
) {
    if *build_mode_type != BuildModeType::RemoveWall {
        return;
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(tile_lookup) = tile_lookup.get_single() {
            let entity: Entity;
            let Vec2 { x, y } = cursor_world_position.position;
            let position = Position::new(x as i32, y as i32);

            if tile_lookup.tiles.contains_key(&position) {
                entity = *tile_lookup.tiles.get(&position).unwrap();

                if wall_query.get(entity).is_err() {
                    println!("Wall does not exist");
                    return;
                }

                if query.get(entity).is_ok() {
                    println!("Remove the object, or item first");
                    return;
                }

                commands.entity(entity).remove::<Wall>();
                commands.entity(entity).insert(Walkable);

                // Remove the child sprite
                // TODO: Find a better way to do this
                for &child in wall_query.get(entity).unwrap().iter() {
                    if sprite_query.get(child).is_ok() {
                        commands.entity(child).despawn_recursive();
                        commands.entity(entity).remove_children(&[child]);
                    }
                }
            }
        }
    }
}
