use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::PresentMode;

use crate::camera::CameraPlugin;
use crate::constants::*;
use crate::cursor::CursorPlugin;
use crate::fps_debug::FPSDebugPlugin;
use crate::game::GamePlugin;
use crate::game_state::GameState;
use crate::main_menu::MainMenuPlugin;
use crate::scene::ScenePlugin;
use crate::splash::SplashPlugin;
use bevy::prelude::*;

pub mod ai;
pub mod animation;
pub mod camera;
pub mod constants;
pub mod cursor;
pub mod fps_debug;
mod game;
pub mod game_state;
pub mod game_ui;
pub mod main_menu;
mod scene;
mod splash;
pub mod tiles;
pub mod utils;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                // This is used for hot reloading assets
                // .set(AssetPlugin {
                //     watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                //     ..default()
                // })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: GAME_TITLE.to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<GameState>()
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(FPSDebugPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(ScenePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(SplashPlugin)
        .add_plugins(GamePlugin)
        .run();
}
