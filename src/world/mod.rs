pub mod collisions;

mod camera;
mod map;

use bevy_rancic::prelude::DebugState;
pub use map::{
    a_star, CachedEnemy, CachedLevelData, DespawnLevelSystemSet, LevelChanged, PathfindingSource,
    PathfindingTarget, WorldSpatialData,
};

use bevy::prelude::*;

use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collisions::WorldCollisionPlugin,
            map::WorldMapPlugin,
            camera::CameraPlugin,
        ))
        .add_systems(OnExit(GameState::AssetLoading), toggle_debug);
    }
}

#[derive(Component)]
pub struct WorldEntity;

fn toggle_debug(mut debug_state: ResMut<DebugState>) {
    **debug_state = !**debug_state;
}
