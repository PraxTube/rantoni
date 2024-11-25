mod animation;
mod collisions;
mod movement;
mod spawn;
mod state;

use bevy::prelude::*;
use state::EnemyStateMachine;

const MAX_CHASE_DISTANCE: f32 = 500.0;
const MIN_CHASE_DISTANCE: f32 = 100.0;
/// Must always be smaller than `MIN_CHASE_DISTANCE`.
/// Acts as a buffer that the player can move while the enemy is still in stalking mode.
/// The bigger the difference the more the enemy can "stalk" the player.
const MIN_TARGET_DISTANCE: f32 = 50.0;
const MOVE_SPEED: f32 = 400.0;
const STALK_SPEED: f32 = 200.0;

pub use collisions::EnemyCollisionSystemSet;

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
    move_direction: Vec2,
    target_pos: Vec2,
    target_offset: Vec2,
    target: Option<Entity>,
    pub state_machine: EnemyStateMachine,
}
