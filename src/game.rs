use crate::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GamePlayState {
    Loading,
    Saving,
    Playing,
    Paused,
    #[default]
    Disabled,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GamePlayState>()
            .add_systems(OnEnter(GameState::Game), game_setup)
            // TODO: Divide GameState::Game into different states
            .add_systems(Update, exit_game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .add_systems(OnEnter(GamePlayState::Loading), spawn_tiles)
            // .add_systems(OnEnter(GamePlayState::Playing), create_tile_lookup)
            .add_systems(
                Update,
                update_sprites.run_if(in_state(GamePlayState::Playing)),
            )
            .add_systems(Update, spawn_actor.run_if(in_state(GamePlayState::Playing)))
            .add_systems(
                Update,
                update_tile_lookup.run_if(in_state(GamePlayState::Playing)),
            )
            .add_systems(
                Update,
                generate_path_for_movers.run_if(in_state(GamePlayState::Playing)),
            )
            .add_systems(
                Update,
                set_target_for_movers.run_if(in_state(GamePlayState::Playing)),
            )
            .add_systems(
                Update,
                move_movers_with_target.run_if(in_state(GamePlayState::Playing)),
            )
            .add_systems(Update, spawn_wall.run_if(in_state(GamePlayState::Playing)))
            .add_systems(
                Update,
                animate_sprite.run_if(in_state(GamePlayState::Playing)),
            );
    }
}

#[derive(Component)]
struct OnGameScreen;

fn game_setup(mut _commands: Commands, mut game_play_state: ResMut<NextState<GamePlayState>>) {
    game_play_state.set(GamePlayState::Loading);
}

fn exit_game(
    mut game_state: ResMut<NextState<GameState>>,
    mut game_play_state: ResMut<NextState<GamePlayState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::MainMenu);
        game_play_state.set(GamePlayState::Paused);
    }
}
