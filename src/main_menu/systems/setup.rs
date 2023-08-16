use crate::main_menu::states::MainMenuState;
use crate::prelude::*;

pub fn setup(mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    main_menu_state.set(MainMenuState::Main);
}
