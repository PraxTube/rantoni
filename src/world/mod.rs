pub mod collisions;

mod camera;
mod debug;
mod map;
mod physics;
mod state;
mod utils;

pub use camera::{MainCamera, ToggleFullscreenEvent, YSort, YSortChild, ZoomCameraScaleEvent};
pub use debug::{DebugState, ToggleDebugStateEvent};
pub use utils::{
    quat_from_vec2, COLLIDER_COLOR_BLACK, COLLIDER_COLOR_WHITE, COLLISION_GROUPS_NONE,
};

pub use map::{
    a_star, CachedEnemy, CachedLevelData, CachedPlayer, DespawnLevelSystemSet, LevelChanged,
    PathfindingSource, PathfindingTarget, WorldSpatialData,
};

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collisions::WorldCollisionPlugin,
            map::WorldMapPlugin,
            camera::CameraPlugin,
            state::WorldStatePlugin,
            physics::WorldPhysicsPlugin,
            debug::DebugPlugin,
        ));
    }
}

#[derive(Component)]
pub struct WorldEntity;
