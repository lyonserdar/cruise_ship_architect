use crate::game::GamePlayState;
use crate::game_ui::components::OnGameUIMainScreen;
use crate::game_ui::resources::BuildModeType;
use crate::game_ui::systems::{game_ui_setup, menu_action};
use crate::main_menu::{handle_button_interaction, handle_selected_button};
use crate::utils::despawn_screen;
use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildModeType::None)
            .add_systems(OnEnter(GamePlayState::Playing), game_ui_setup)
            .add_systems(
                OnExit(GamePlayState::Playing),
                despawn_screen::<OnGameUIMainScreen>,
            )
            .add_systems(
                Update,
                (
                    menu_action,
                    handle_button_interaction,
                    handle_selected_button::<BuildModeType>,
                )
                    .run_if(in_state(GamePlayState::Playing)),
            );
    }
}
