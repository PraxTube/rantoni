pub mod input;

mod animation;
mod collisions;
mod movement;
mod spawn;
mod state;

#[allow(unused)]
pub use state::PlayerStateSystemSet;

use bevy::prelude::*;
use state::PlayerStateMachine;

pub const HEALTH: u32 = 100;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            spawn::PlayerSpawnPlugin,
            movement::PlayerMovementPlugin,
            animation::PlayerAnimationPlugin,
            state::PlayerStatePlugin,
            collisions::PlayerCollisionsPlugin,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub state_machine: PlayerStateMachine,
    pub current_direction: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state_machine: PlayerStateMachine::default(),
            current_direction: Vec2::NEG_Y,
        }
    }
}
