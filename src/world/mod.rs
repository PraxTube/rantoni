pub mod collisions;

mod camera;
mod map;
mod state;

pub use map::{
    a_star, CachedEnemy, CachedLevelData, DespawnLevelSystemSet, LevelChanged, PathfindingSource,
    PathfindingTarget, WorldSpatialData,
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
        ));
    }
}

#[derive(Component)]
pub struct WorldEntity;
