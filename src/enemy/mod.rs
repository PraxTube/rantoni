mod animation;
mod collisions;
mod movement;
mod spawn;
mod state;

use bevy::prelude::*;
use state::EnemyStateMachine;

use crate::state::Stagger;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::EnemyStatePlugin,
            movement::EnemyMovementPlugin,
            collisions::EnemyCollisionsPlugin,
            spawn::EnemySpawnPlugin,
            animation::EnemyAnimationPlugin,
        ));
    }
}

#[derive(Component, Default)]
pub struct Enemy {
    state_machine: EnemyStateMachine,
    stagger: Stagger,
}
