mod components;
pub mod plugins;
mod resources;
mod states;
mod styles;
mod systems;

pub use components::SelectedOption;
pub use plugins::MainMenuPlugin;
pub use systems::handle_button_interaction::handle_button_interaction;
pub use systems::handle_selected_button::handle_selected_button;
