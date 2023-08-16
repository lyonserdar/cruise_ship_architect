use crate::main_menu::components::{MainMenuButtonAction, OnMainMenuSettingsScreen};
use crate::main_menu::styles::{
    button_text_style, main_container, menu_container, BUTTON_STYLE, NORMAL_BUTTON,
};
use crate::prelude::*;

pub fn settings_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((main_container(), OnMainMenuSettingsScreen))
        .with_children(|parent| {
            parent.spawn(menu_container()).with_children(|parent| {
                for (action, text) in [
                    (MainMenuButtonAction::SettingsDisplay, "Display"),
                    (MainMenuButtonAction::SettingsSound, "Sound"),
                    (MainMenuButtonAction::BackToMainMenu, "Back"),
                ] {
                    parent
                        .spawn((
                            ButtonBundle {
                                // style: button_style.clone(),
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            action,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                text,
                                // button_text_style.clone(),
                                button_text_style(&asset_server),
                            ));
                        });
                }
            });
        });
}
