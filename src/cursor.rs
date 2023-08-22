use crate::camera::MainCamera;
use crate::constants::TILE_SIZE;
use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cursor_position)
            .insert_resource(CursorPosition::default())
            .insert_resource(CursorWorldPosition::default());
    }
}

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub position: Vec2,
}

#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    pub position: Vec2,
}

pub fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    mut cursor_world_position: ResMut<CursorWorldPosition>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        cursor_position.position = Vec2::new(world_position.x, world_position.y);
        cursor_world_position.position = Vec2::new(
            (world_position.x / TILE_SIZE as f32).round() as i32 as f32,
            (world_position.y / TILE_SIZE as f32).round() as i32 as f32,
        );
    }
}
