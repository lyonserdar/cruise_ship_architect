use bevy::prelude::*;

#[derive(Component)]
pub struct OnGameUIMainScreen;

#[derive(Component)]
pub struct SelectedGameUIOption;

#[derive(Component)]
pub enum GameUIButtonAction {
    SelectBuildModeType,
}
