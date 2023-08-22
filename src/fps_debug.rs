use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

use bevy::prelude::*;

pub struct FPSDebugPlugin;

impl Plugin for FPSDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_text);
        app.add_systems(Startup, spawn_timers);
        app.add_systems(Update, update_fps_text);
    }
}

#[derive(Component)]
struct FPSText;

#[derive(Component)]
struct FPSTextColor;

#[derive(Component)]
struct FPSUpdateCountdown(Timer);

fn spawn_timers(mut commands: Commands) {
    commands.spawn(FPSUpdateCountdown(Timer::from_seconds(
        1.,
        TimerMode::Repeating,
    )));
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/conthrax-sb.otf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/conthrax-sb.otf"),
                font_size: 20.0,
                color: Color::WHITE,
            }),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        FPSText,
    ));
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FPSText>>,
    time: Res<Time>,
    mut timer_query: Query<&mut FPSUpdateCountdown>,
) {
    for mut timer in &mut timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            let mut text = query.single_mut();
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    text.sections[1].value = (value.round() as u64).to_string();
                }
            }
        }
    }
}
