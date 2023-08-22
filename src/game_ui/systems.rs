use crate::constants::TEXT_COLOR;
use crate::game::OnGameScreen;
use crate::game_ui::components::{GameUIButtonAction, OnGameUIMainScreen};
use crate::game_ui::resources::BuildModeType;
use crate::main_menu::SelectedOption;
use bevy::prelude::*;

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &GameUIButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                GameUIButtonAction::SelectBuildModeType => {
                    println!("SelectBuildModeType");
                }
            }
        }
    }
}

pub fn game_ui_setup(mut commands: Commands) {
    let button_style = Style {
        // width: Val::Px(100.0),
        height: Val::Px(30.0),
        // margin: UiRect::all(Val::Px(5.0)),
        padding: UiRect::all(Val::Px(10.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 20.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },

                background_color: Color::rgba(0., 0., 0., 0.25).into(),
                ..default()
            },
            OnGameUIMainScreen,
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for build_type in [
                        BuildModeType::None,
                        BuildModeType::Floor,
                        BuildModeType::Wall,
                        BuildModeType::Object,
                        BuildModeType::Item,
                        BuildModeType::RemoveFloor,
                        BuildModeType::RemoveWall,
                        BuildModeType::RemoveObject,
                        BuildModeType::RemoveItem,
                    ] {
                        let mut entity = parent.spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            GameUIButtonAction::SelectBuildModeType,
                        ));

                        entity.insert(build_type).with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    format!("{build_type:?}"),
                                    button_text_style.clone(),
                                ),
                                ..default()
                            });
                        });

                        if build_type == BuildModeType::None {
                            entity.insert(SelectedOption);
                        }
                    }
                });
        });
}
