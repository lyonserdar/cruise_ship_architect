use crate::game_state::GameState;
use crate::main_menu::components::MainMenuButtonAction;
use crate::main_menu::states::MainMenuState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainMenuButtonAction::Quit => app_exit_events.send(AppExit),
                MainMenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MainMenuState::Disabled);
                }
                MainMenuButtonAction::Settings => menu_state.set(MainMenuState::Settings),
                MainMenuButtonAction::SettingsDisplay => {
                    menu_state.set(MainMenuState::SettingsDisplay);
                }
                MainMenuButtonAction::SettingsSound => {
                    menu_state.set(MainMenuState::SettingsSound);
                }
                MainMenuButtonAction::BackToMainMenu => menu_state.set(MainMenuState::Main),
                MainMenuButtonAction::BackToSettings => {
                    menu_state.set(MainMenuState::Settings);
                }
            }
        }
    }
}
