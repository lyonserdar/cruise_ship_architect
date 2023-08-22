use crate::ai::components::actor::Actor;
use crate::ai::movers::{Mover, MoverState};
use crate::animation::{AnimationIndices, AnimationStates, AnimationTimer, Facing};
use crate::constants::ACTOR_SPAWN_COUNT;
use bevy::prelude::*;

pub fn spawn_actor(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if keyboard_input.just_pressed(KeyCode::U) {
        let texture_handle = asset_server.load("sprites/character.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16., 16.),
            4,
            8,
            None,
            None, // Some(Vec2::new(0., 32. * 11.)),
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices {
            idle_up: (8, 11),
            idle_down: (0, 3),
            idle_left: (4, 7),
            idle_right: (12, 15),
            walk_up: (24, 27),
            walk_down: (16, 19),
            walk_left: (20, 23),
            walk_right: (28, 31),
        };

        for _ in 0..ACTOR_SPAWN_COUNT {
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(animation_indices.walk_down.0),
                    transform: Transform::from_xyz(
                        // (rand::thread_rng().gen_range(0..GRID_WIDTH) * TILE_SIZE) as f32,
                        // (rand::thread_rng().gen_range(0..GRID_HEIGHT) * TILE_SIZE) as f32,
                        // 0.,
                        0., 0., 1.,
                    ),
                    ..Default::default()
                },
                Actor,
                Mover::new(100.0),
                MoverState::default(),
                Facing::default(),
                animation_indices,
                AnimationStates::default(),
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ));
        }
    }
}
