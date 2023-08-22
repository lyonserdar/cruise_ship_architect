use crate::main_menu::states::MainMenuState;
use bevy::prelude::*;

pub fn setup(mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    main_menu_state.set(MainMenuState::Main);
}
