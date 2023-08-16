use crate::main_menu::components::{
    MainMenuButtonAction, OnMainMenuSoundSettingsScreen, SelectedOption,
};
use crate::main_menu::styles::{
    button_text_style, main_container, menu_container, BUTTON_STYLE, NORMAL_BUTTON,
};
use crate::prelude::*;

pub fn sound_settings_menu_setup(
    mut commands: Commands,
    volume: Res<Volume>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((main_container(), OnMainMenuSoundSettingsScreen))
        .with_children(|parent| {
            parent.spawn(menu_container()).with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Volume",
                            // button_text_style.clone(),
                            button_text_style(&asset_server),
                        ));
                        for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                            let mut entity = parent.spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(65.0),
                                    ..BUTTON_STYLE // ..button_style.clone()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            });
                            entity.insert(Volume(volume_setting));
                            if *volume == Volume(volume_setting) {
                                entity.insert(SelectedOption);
                            }
                        }
                    });
                parent
                    .spawn((
                        ButtonBundle {
                            style: BUTTON_STYLE,
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        MainMenuButtonAction::BackToSettings,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Back",
                            button_text_style(&asset_server),
                        ));
                    });
            });
        });
}
