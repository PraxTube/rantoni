pub use bevy::prelude::*;

use crate::{dude::Health, player::Player, GameState};

fn transition_game_over_state(
    mut next_state: ResMut<NextState<GameState>>,
    q_players: Query<&Health, With<Player>>,
) {
    for health in &q_players {
        if health.health == 0 {
            next_state.set(GameState::GameOverPadding);
        }
    }
}

pub struct WorldStatePlugin;

impl Plugin for WorldStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            transition_game_over_state.run_if(in_state(GameState::Gaming)),
        );
    }
}
