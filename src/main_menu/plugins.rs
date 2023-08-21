use crate::main_menu::components::{
    OnMainMenuDisplaySettingsScreen, OnMainMenuMainScreen, OnMainMenuSettingsScreen,
    OnMainMenuSoundSettingsScreen,
};
use crate::main_menu::resources::{DisplayQuality, Volume};
use crate::main_menu::states::MainMenuState;
use crate::main_menu::systems::display_settings_setup::display_settings_menu_setup;
use crate::main_menu::systems::handle_button_interaction::handle_button_interaction;
use crate::main_menu::systems::handle_selected_button::handle_selected_button;
use crate::main_menu::systems::main_setup::main_setup;
use crate::main_menu::systems::menu_action::menu_action;
use crate::main_menu::systems::settings_setup::settings_setup;
use crate::main_menu::systems::setup::setup;
use crate::main_menu::systems::sound_settings_setup::sound_settings_menu_setup;
use crate::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .insert_resource(DisplayQuality::High)
            .insert_resource(Volume(7))
            .add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(
                Update,
                (menu_action, handle_button_interaction).run_if(in_state(GameState::MainMenu)),
            )
            // Main
            .add_systems(OnEnter(MainMenuState::Main), main_setup)
            .add_systems(
                OnExit(MainMenuState::Main),
                despawn_screen::<OnMainMenuMainScreen>,
            )
            // Settings
            .add_systems(OnEnter(MainMenuState::Settings), settings_setup)
            .add_systems(
                OnExit(MainMenuState::Settings),
                despawn_screen::<OnMainMenuSettingsScreen>,
            )
            // Display Settings
            .add_systems(
                OnEnter(MainMenuState::SettingsDisplay),
                display_settings_menu_setup,
            )
            .add_systems(
                Update,
                (handle_selected_button::<DisplayQuality>
                    .run_if(in_state(MainMenuState::SettingsDisplay)),),
            )
            .add_systems(
                OnExit(MainMenuState::SettingsDisplay),
                despawn_screen::<OnMainMenuDisplaySettingsScreen>,
            )
            // Sound Settings
            .add_systems(
                OnEnter(MainMenuState::SettingsSound),
                sound_settings_menu_setup,
            )
            .add_systems(
                Update,
                handle_selected_button::<Volume>.run_if(in_state(MainMenuState::SettingsSound)),
            )
            .add_systems(
                OnExit(MainMenuState::SettingsSound),
                despawn_screen::<OnMainMenuSoundSettingsScreen>,
            );
    }
}
