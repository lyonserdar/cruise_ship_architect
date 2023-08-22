use crate::ai::movers::systems::generate_path_for_movers::generate_path_for_movers;
use crate::ai::movers::systems::move_movers_with_target::move_movers_with_target;
use crate::ai::movers::systems::set_target_for_movers::set_target_for_movers;
use crate::game::GamePlayState;
use bevy::prelude::*;

pub struct MoversPlugin;

impl Plugin for MoversPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
        );
    }
}
