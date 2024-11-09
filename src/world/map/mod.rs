mod level_transition;
mod pathfinding;

use level_transition::LevelChangeDirection;
pub use level_transition::LevelChanged;

use bevy_rancic::prelude::DebugState;
pub use pathfinding::a_star;

use std::{fs, str::from_utf8};

use bevy::{color::palettes::css::*, prelude::*, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;

use generate_world_collisions::{deserialize_polygons, MAP_POLYGON_DATA};

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_plugins((level_transition::MapLevelTransition,))
            .insert_resource(LevelSelection::indices(0, 0))
            .add_systems(
                OnExit(GameState::AssetLoading),
                (spawn_ldtk_world, deserialize_and_insert_wrold_data),
            )
            .add_systems(
                Update,
                (debug_enemy_pathfinding.run_if(resource_exists::<WorldSpatialData>),),
            );
    }
}

#[derive(Component)]
pub struct PathfindingSource {
    pub root_entity: Entity,
    pub target: Option<Entity>,
    pub path: Option<Vec<Vec2>>,
}
#[derive(Component)]
pub struct PathfindingTarget {
    pub root_entity: Entity,
}

#[derive(Resource, Debug)]
pub struct WorldSpatialData {
    pub levels_spatial_data: HashMap<(usize, usize), LevelSpatialData>,
    current_level: (usize, usize),
}

#[derive(Debug)]
pub struct LevelSpatialData {
    pub grid_matrix: Vec<Vec<u8>>,
    pub collider_polygons: Vec<Vec<Vec2>>,
    pub neighbours: [Option<(usize, i32, i32)>; 4],
}

impl PathfindingSource {
    pub fn new(root_entity: Entity) -> Self {
        Self {
            root_entity,
            target: None,
            path: None,
        }
    }
}

impl WorldSpatialData {
    pub fn current_level(&self) -> &LevelSpatialData {
        match self.levels_spatial_data.get(&self.current_level) {
            Some(level) => level,
            None => panic!("should never happen, world: {:?}", self),
        }
    }

    pub fn grid_matrix(&self) -> &Vec<Vec<u8>> {
        &self.current_level().grid_matrix
    }

    pub fn collider_polygons(&self) -> &Vec<Vec<Vec2>> {
        &self.current_level().collider_polygons
    }

    pub fn level_dimensions(&self) -> UVec2 {
        let level = self.current_level();
        UVec2::new(
            level.grid_matrix.len() as u32,
            level.grid_matrix[0].len() as u32,
        )
    }

    pub fn transition_level_and_get_offset(
        &mut self,
        direction: LevelChangeDirection,
    ) -> (i32, i32) {
        assert_ne!(direction, LevelChangeDirection::None);

        match self.current_level().neighbours[direction.to_usize()] {
            Some(neighbour) => {
                self.current_level = (self.current_level.0, neighbour.0);
                (neighbour.1, neighbour.2)
            }
            None => {
                panic!("trying to transition to neighbour that does not exist, your map is wrong!, current: {:?}, next: {:?}, dir: {}", self.current_level, self.current_level().neighbours, direction.to_usize())
            }
        }
    }
}

fn spawn_ldtk_world(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.map.clone(),
        transform: Transform::from_translation(Vec3::Z * Z_LEVEL_BACKGROUND),
        ..default()
    });
}

fn deserialize_and_insert_wrold_data(mut commands: Commands) {
    let serialized_buffer = fs::read(MAP_POLYGON_DATA).expect("failed to read map polygon data");
    let serialized_data =
        from_utf8(&serialized_buffer).expect("invalid UTF-8 sequence in map polygon data");
    let raw_levels_spatial_data = deserialize_polygons(serialized_data);

    let mut levels_spatial_data = HashMap::new();
    for level_data in raw_levels_spatial_data {
        levels_spatial_data.insert(
            level_data.0,
            LevelSpatialData {
                grid_matrix: level_data.1,
                collider_polygons: level_data.2,
                neighbours: level_data.3,
            },
        );
    }

    commands.insert_resource(WorldSpatialData {
        levels_spatial_data,
        current_level: (0, 0),
    });
}

fn debug_enemy_pathfinding(
    mut gizmos: Gizmos,
    debug_state: Res<DebugState>,
    world_data: Res<WorldSpatialData>,
    q_player: Query<&GlobalTransform, With<PathfindingTarget>>,
    q_enemies: Query<&GlobalTransform, (With<PathfindingSource>, Without<PathfindingTarget>)>,
) {
    if !debug_state.0 {
        return;
    }
    let Ok(player_transform) = q_player.get_single() else {
        return;
    };

    for enemy_transform in &q_enemies {
        let start = enemy_transform.translation().truncate();
        let end = player_transform.translation().truncate();

        let mut path = a_star(start, end, world_data.grid_matrix(), &None);

        path.insert(0, start);
        path.push(end);

        for i in 0..path.len() - 1 {
            let color = Srgba::new(
                LIGHT_GREEN.red,
                LIGHT_GREEN.green * i as f32 / (path.len() - 1) as f32,
                LIGHT_GREEN.blue,
                1.0,
            );
            gizmos.line_2d(path[i], path[i + 1], color);
        }
    }
}
