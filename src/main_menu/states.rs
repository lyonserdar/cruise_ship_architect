use crate::prelude::*;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum MainMenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}
