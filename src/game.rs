use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            // TODO: Divide GameState::Game into different states
            // TODO: this shouldn't be global resource. It should be added and removed when needed
            .insert_resource(Grid::default())
            .add_systems(Update, exit_game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            // .add_systems(Startup, spawn_tiles)
            // .add_systems(Update, spawn_actor)
            // .add_systems(Update, generate_path_for_movers)
            // .add_systems(Update, set_target_for_movers)
            // .add_systems(Update, move_movers_with_target)
            // .add_systems(Update, spawn_wall)
            // .add_systems(Update, animate_sprite)
            .add_systems(OnEnter(GameState::Game), spawn_tiles.after(game_setup))
            .add_systems(Update, spawn_actor.run_if(in_state(GameState::Game)))
            .add_systems(
                Update,
                generate_path_for_movers.run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                set_target_for_movers.run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                move_movers_with_target.run_if(in_state(GameState::Game)),
            )
            .add_systems(Update, spawn_wall.run_if(in_state(GameState::Game)))
            .add_systems(Update, animate_sprite.run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
struct OnGameScreen;

fn game_setup(mut commands: Commands) {
    println!("Game setup");
}

fn exit_game(mut game_state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::MainMenu);
    }
}
