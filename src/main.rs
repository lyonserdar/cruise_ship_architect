use bevy::asset::ChangeWatcher;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::PresentMode;

use crate::game::GamePlugin;
use crate::prelude::*;
use crate::splash::SplashPlugin;

pub mod actor;
pub mod animation;
pub mod camera;
pub mod constants;
pub mod cursor;
pub mod fps_debug;
mod game;
pub mod game_state;
pub mod main_menu;
pub mod mover;
pub mod node;
pub mod position;
pub mod prelude;
mod scene;
mod splash;
pub mod tile;
pub mod utils;

// fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
//     let main_menu_entity = commands
//         .spawn(NodeBundle {
//             background_color: Color::RED.into(),
//             ..default()
//         })
//         .id();
//
//     main_menu_entity
// }

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Cruise Ship Architect"),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(DisplayQuality::High)
        .insert_resource(Volume(7))
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

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

pub fn button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut _text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, _children) in interaction_query.iter_mut() {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Pressed".to_string();
                *color = Color::RED.into();
                border_color.0 = Color::YELLOW;
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
                border_color.0 = Color::BLUE;
            }
            Interaction::None => {
                *color = Color::BLACK.into();
                border_color.0 = Color::WHITE;
            }
        }
    }
}

pub fn spawn_ui(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    });
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             width: Val::Percent(100.0),
    //             align_items: AlignItems::FlexEnd,
    //             justify_content: JustifyContent::FlexStart,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent
    //             .spawn(ButtonBundle {
    //                 style: Style {
    //                     width: Val::Px(200.0),
    //                     height: Val::Px(100.0),
    //                     border: UiRect::all(Val::Px(5.0)),
    //                     // horizontally center child text
    //                     justify_content: JustifyContent::Center,
    //                     // vertically center child text
    //                     align_items: AlignItems::Center,
    //                     ..default()
    //                 },
    //                 border_color: BorderColor(Color::BLACK),
    //                 background_color: Color::BLACK.into(),
    //                 ..default()
    //             })
    //             .with_children(|parent| {
    //                 parent.spawn(TextBundle::from_section(
    //                     "Button",
    //                     TextStyle {
    //                         font: asset_server.load("fonts/conthrax-sb.otf"),
    //                         font_size: 40.0,
    //                         color: Color::rgb(0.9, 0.9, 0.9),
    //                     },
    //                 ));
    //             });
    //     });
}
