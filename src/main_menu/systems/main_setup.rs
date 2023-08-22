use crate::constants::GAME_TITLE;
use crate::main_menu::components::{MainMenuButtonAction, OnMainMenuMainScreen};
use crate::main_menu::styles::{
    button_text_style, main_container, menu_container, BUTTON_STYLE, NORMAL_BUTTON,
};
use bevy::prelude::*;

pub fn main_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((main_container(), OnMainMenuMainScreen))
        .with_children(|parent| {
            parent.spawn(menu_container()).with_children(|parent| {
                // Display the game name
                parent.spawn(
                    TextBundle::from_section(
                        GAME_TITLE,
                        TextStyle {
                            font_size: 80.0,
                            ..button_text_style(&asset_server) // ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    }),
                );

                // Display three buttons for each action available from the main menu:
                // - new game
                // - settings
                // - quit
                for (action, text) in [
                    (MainMenuButtonAction::Play, "New Game"),
                    (MainMenuButtonAction::Settings, "Settings"),
                    (MainMenuButtonAction::Quit, "Quit"),
                ] {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            action,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                text,
                                button_text_style(&asset_server),
                            ));
                        });
                }
            });
        });
}
