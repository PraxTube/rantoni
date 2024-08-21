pub mod input;

mod animation;
mod movement;
mod spawn;
mod state;

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

#[derive(Component)]
pub struct Player {
    pub state_machine: PlayerStateMachine,
    pub punching_direction: Vec2,
}

impl FromWorld for Player {
    fn from_world(world: &mut World) -> Self {
        Self {
            state_machine: PlayerStateMachine::from_world(world),
            punching_direction: Vec2::ZERO,
        }
    }
}
