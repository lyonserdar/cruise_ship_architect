use crate::constants::TEXT_COLOR;
use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(300.0);
    style.height = Val::Px(80.0);
    // style.padding = UiRect::all(Val::Px(20.0));
    style.margin = UiRect::all(Val::Px(20.0));
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub fn button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/conthrax-sb.otf"),
        font_size: 40.0,
        color: TEXT_COLOR,
    }
}

pub fn main_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // background_color: Color::rgba(0., 0., 0., 0.50).into(),
        background_color: Color::BLACK.into(),
        ..default()
    }
}

pub fn menu_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    }
}
