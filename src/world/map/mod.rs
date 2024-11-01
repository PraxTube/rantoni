use std::{fs, str::from_utf8};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use generate_world_collisions::{deserialize_polygons, MAP_POLYGON_DATA};

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(
                OnEnter(GameState::Gaming),
                (spawn_ldtk_world, parse_polygon_data),
            );
    }
}

#[derive(Resource)]
pub struct MapPolygonData {
    pub _navmesh_polygons: Vec<Vec<Vec2>>,
    pub collider_polygons: Vec<Vec<Vec2>>,
}

fn spawn_ldtk_world(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.map.clone(),
        transform: Transform::from_translation(Vec3::Z * Z_LEVEL_BACKGROUND),
        ..default()
    });
}

fn parse_polygon_data(mut commands: Commands) {
    let serialized_buffer = fs::read(MAP_POLYGON_DATA).expect("failed to read map polygon data");
    let serialized_data =
        from_utf8(&serialized_buffer).expect("invalid UTF-8 sequence in map polygon data");
    let (navmesh_polygons, collider_polygons) = deserialize_polygons(serialized_data);

    commands.insert_resource(MapPolygonData {
        _navmesh_polygons: navmesh_polygons,
        collider_polygons,
    });
}
