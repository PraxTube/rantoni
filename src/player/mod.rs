pub mod input;

mod animation;
mod collisions;
mod movement;
mod spawn;
mod state;

pub use spawn::PlayerHitboxRoot;
pub use state::PlayerStateSystemSet;

use bevy::prelude::*;
use state::PlayerStateMachine;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            collisions::PlayerCollisionsPlugin,
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
    pub aim_direction: Vec2,
    pub current_direction: Vec2,
}
