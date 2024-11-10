pub mod collisions;

mod camera;
mod map;

pub use map::{
    a_star, DespawnLevelSystemSet, LevelChanged, PathfindingSource, PathfindingTarget,
    WorldSpatialData,
};

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collisions::WorldCollisionPlugin,
            map::WorldMapPlugin,
            camera::CameraPlugin,
        ));
    }
}

#[derive(Component)]
pub struct WorldEntity;
