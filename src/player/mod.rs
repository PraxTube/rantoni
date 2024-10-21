pub mod input;

mod animation;
mod movement;
mod spawn;
mod state;

#[allow(unused)]
pub use state::PlayerStateSystemSet;

use bevy::prelude::*;
use state::PlayerStateMachine;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            spawn::PlayerSpawnPlugin,
            movement::PlayerMovementPlugin,
            animation::PlayerAnimationPlugin,
            state::PlayerStatePlugin,
        ));
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub state_machine: PlayerStateMachine,
    pub current_direction: Vec2,
}
