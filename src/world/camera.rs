use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_rancic::prelude::*;
use generate_world_collisions::TILE_SIZE;

use crate::player::Player;

use super::{LevelChanged, WorldSpatialData};

fn update_camera_target(
    mut camera_settings: ResMut<CameraSettings>,
    q_player: Query<&Transform, With<Player>>,
) {
    let transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    camera_settings.update_target(transform.translation.truncate());
}

fn update_camera_bounds(
    mut camera_settings: ResMut<CameraSettings>,
    world_data: Res<WorldSpatialData>,
) {
    let level_dim = world_data.level_dimensions();
    let max = Vec2::new(level_dim.x as f32 - 1.0, level_dim.y as f32 - 1.0) * TILE_SIZE;
    camera_settings.set_bound(Aabb2d {
        min: Vec2::ZERO,
        max,
    });
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                update_camera_target.in_set(CameraSystem::TargetUpdate),
                update_camera_bounds.run_if(on_event::<LevelChanged>()),
            ),
        );
    }
}
