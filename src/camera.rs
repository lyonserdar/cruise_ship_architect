use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::render::view::RenderLayers;

use crate::constants::{GRID_HEIGHT, GRID_WIDTH, TILE_SIZE};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_movement)
            .add_systems(Update, camera_zoom);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(
                (GRID_WIDTH * TILE_SIZE) as f32 / 2.,
                (GRID_HEIGHT * TILE_SIZE) as f32 / 2.,
                100.,
            )),
            ..default()
        },
        MainCamera,
        RenderLayers::all(),
    ));
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 100. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 100. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 100. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 100. * time.delta_seconds();
        }
    }
}

fn camera_zoom(
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let scroll = scroll_events
        .iter()
        .map(|e| match e.unit {
            MouseScrollUnit::Pixel => e.y,
            MouseScrollUnit::Line => e.y * 100., // TODO: make it configurable
        })
        .sum::<f32>();

    if scroll == 0.0 {
        return;
    }

    for mut projection in query.iter_mut() {
        projection.scale = projection.scale * (1. + -scroll * 0.001);
        projection.scale = projection.scale.clamp(0.1, 2.5);
    }
}
