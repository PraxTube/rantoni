mod animation;
mod collisions;
mod crowd;
mod movement;
mod spawn;
mod state;

use bevy::prelude::*;
use state::EnemyStateMachine;

const MAX_CHASE_DISTANCE: f32 = 1000.0;
const MIN_CHASE_DISTANCE: f32 = 200.0;
/// Must always be smaller than `MIN_CHASE_DISTANCE`.
/// Acts as a buffer that the player can move while the enemy is still in stalking mode.
/// The bigger the difference the more the enemy can "stalk" the player.
const ATTACK_DISTANCE: f32 = 75.0;
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
            crowd::EnemyCrowdPlugin,
        ));
    }
}

#[derive(Component, Default)]
pub struct Enemy {
    move_direction: Vec2,
    /// The position of the next point to move to.
    /// This can be any point in the game, it will usual be points on the path,
    /// generated by the pathfinding algorithm.
    /// If there is a clear line of sight then this point will be the same as `target_pos`.
    move_target_pos: Vec2,
    target: Option<Entity>,
    /// Point of the actual current target of this enemy.
    target_pos: Vec2,
    /// Random offset, this may need to be removed as it's not a great solution.
    target_offset: Vec2,
    pub state_machine: EnemyStateMachine,
}
