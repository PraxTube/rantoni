pub mod collisions;

mod map;

pub use map::{a_star, LevelChanged, PathfindingSource, PathfindingTarget, WorldSpatialData};

use bevy::prelude::*;
use bevy_rancic::prelude::*;

use crate::player::Player;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((collisions::WorldCollisionPlugin, map::WorldMapPlugin))
            .add_systems(
                PostUpdate,
                update_camera_target.in_set(CameraSystem::TargetUpdate),
            );
    }
}

fn update_camera_target(
    mut camera_shake: ResMut<CameraShake>,
    q_player: Query<&Transform, With<Player>>,
) {
    let transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    camera_shake.update_target(transform.translation.truncate());
}
