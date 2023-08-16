use crate::prelude::*;

#[derive(Component)]
pub struct OnMainMenuMainScreen;

#[derive(Component)]
pub struct OnMainMenuSettingsScreen;

#[derive(Component)]
pub struct OnMainMenuDisplaySettingsScreen;

#[derive(Component)]
pub struct OnMainMenuSoundSettingsScreen;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub enum MainMenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}
