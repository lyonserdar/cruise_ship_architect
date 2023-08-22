use crate::main_menu::components::SelectedOption;
use crate::main_menu::styles::{
    HOVERED_BUTTON, HOVERED_PRESSED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
};
use bevy::prelude::*;

// This system handles changing all buttons color based on mouse interaction
pub fn handle_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in interaction_query.iter_mut() {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
