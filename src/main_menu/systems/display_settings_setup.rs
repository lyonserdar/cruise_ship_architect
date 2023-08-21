use crate::main_menu::components::{
    MainMenuButtonAction, OnMainMenuDisplaySettingsScreen, SelectedOption,
};
use crate::main_menu::resources::DisplayQuality;
use crate::main_menu::styles::{
    button_text_style, main_container, menu_container, BUTTON_STYLE, NORMAL_BUTTON,
};
use crate::prelude::*;

pub fn display_settings_menu_setup(
    mut commands: Commands,
    display_quality: Res<DisplayQuality>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((main_container(), OnMainMenuDisplaySettingsScreen))
        .with_children(|parent| {
            parent.spawn(menu_container()).with_children(|parent| {
                // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                // use the default value, `FlexDirection::Row`, from left to right.
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
                        // Display a label for the current setting
                        parent.spawn(TextBundle::from_section(
                            "Display Quality",
                            // button_text_style.clone(),
                            button_text_style(&asset_server),
                        ));
                        // Display a button for each possible value
                        for quality_setting in [
                            DisplayQuality::Low,
                            DisplayQuality::Medium,
                            DisplayQuality::High,
                        ] {
                            let mut entity = parent.spawn(ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            });
                            entity.insert(quality_setting).with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    format!("{quality_setting:?}"),
                                    // button_text_style.clone(),
                                    button_text_style(&asset_server),
                                ));
                            });
                            if *display_quality == quality_setting {
                                entity.insert(SelectedOption);
                            }
                        }
                    });
                // Display the back button to return to the settings screen
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
