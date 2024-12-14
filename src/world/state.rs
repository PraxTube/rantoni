pub use bevy::prelude::*;

use crate::{
    dude::Health,
    player::{input::MenuInput, Player},
    GameState,
};

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

fn transition_restart_state(
    mut next_state: ResMut<NextState<GameState>>,
    menu_input: Res<MenuInput>,
) {
    if menu_input.restart {
        next_state.set(GameState::Restart);
    }
}

fn transition_gaming_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Gaming);
}

pub struct WorldStatePlugin;

impl Plugin for WorldStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                transition_game_over_state.run_if(in_state(GameState::Gaming)),
                transition_restart_state.run_if(in_state(GameState::GameOver)),
            ),
        )
        .add_systems(OnEnter(GameState::Restart), transition_gaming_state);
    }
}
