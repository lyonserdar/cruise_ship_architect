use crate::cursor::CursorWorldPosition;
use crate::game::OnGameScreen;
use crate::game_ui::BuildModeType;
use crate::tiles::bundles::TileBundle;
use crate::tiles::components::floor::Floor;
use crate::tiles::components::position::Position;
use crate::tiles::components::tile::Tile;
use crate::tiles::components::tile_lookup::TileLookup;
use crate::tiles::components::walkable::Walkable;
use bevy::prelude::*;

pub fn spawn_floor(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    cursor_world_position: Res<CursorWorldPosition>,
    tile_lookup: Query<&TileLookup>,
    floor_query: Query<Entity, (With<Tile>, With<Floor>)>,
    build_mode_type: Res<BuildModeType>,
) {
    if *build_mode_type != BuildModeType::Floor {
        return;
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(tile_lookup) = tile_lookup.get_single() {
            let entity: Entity;
            let Vec2 { x, y } = cursor_world_position.position;
            let position = Position::new(x as i32, y as i32);

            if tile_lookup.tiles.contains_key(&position) {
                entity = *tile_lookup.tiles.get(&position).unwrap();

                if floor_query.get(entity).is_ok() {
                    println!("Floor already exists.");
                    return;
                }
            } else {
                entity = commands
                    .spawn((
                        TileBundle {
                            position,
                            ..default()
                        },
                        OnGameScreen,
                    ))
                    .id();
            }

            commands.entity(entity).insert(Walkable).insert(Floor);
        }
    }
}
