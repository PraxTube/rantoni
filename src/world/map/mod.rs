mod debug;
mod level_transition;
mod pathfinding;

pub use level_transition::{DespawnLevelSystemSet, LevelChanged};
pub use pathfinding::a_star;

use std::{fs, str::from_utf8};

use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;

use generate_world_collisions::{deserialize_polygons, MAP_POLYGON_DATA, TILE_SIZE};
use level_transition::LevelChangeDirection;

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_plugins((level_transition::MapLevelTransition, debug::MapDebugPlugin))
            .insert_resource(LevelSelection::indices(0, 0))
            .add_systems(
                OnExit(GameState::AssetLoading),
                (spawn_ldtk_world, deserialize_and_insert_wrold_data),
            )
            .add_systems(PreUpdate, update_pf_source_target_positions);
    }
}

#[derive(Component)]
pub struct PathfindingSource {
    pub root_entity: Entity,
    pub target: Option<Entity>,
    pub target_pos: Vec2,
    pub path: Option<Vec<Vec2>>,
}
#[derive(Component)]
pub struct PathfindingTarget {
    pub root_entity: Entity,
}

#[derive(Resource, Debug)]
pub struct WorldSpatialData {
    levels_spatial_data: HashMap<(usize, usize), LevelSpatialData>,
    current_level: (usize, usize),
    previous_level: Option<(usize, usize)>,
    level_transition_offset: IVec2,
    level_transition_direction: LevelChangeDirection,
    cached_player: Option<CachedPlayer>,
}

#[derive(Debug)]
pub struct LevelSpatialData {
    grid_matrix: Vec<Vec<u8>>,
    collider_polygons: Vec<Vec<Vec2>>,
    neighbours: [Option<(usize, i32, i32)>; 4],
    cached_data: Option<CachedLevelData>,
}

#[derive(Debug, Clone, Default)]
pub struct CachedLevelData {
    pub enemies: Vec<CachedEnemy>,
    pub bloodpiles: Vec<Vec2>,
}

#[derive(Debug, Clone)]
pub struct CachedPlayer {
    pub pos: Vec2,
    pub health: u32,
}

#[derive(Debug, Clone)]
pub struct CachedEnemy {
    pub pos: Vec2,
    // TODO: Cache health of enemy
    // pub health: u32,
}

impl PathfindingSource {
    pub fn new(root_entity: Entity) -> Self {
        Self {
            root_entity,
            target: None,
            target_pos: Vec2::ZERO,
            path: None,
        }
    }
}

impl WorldSpatialData {
    fn current_spatial_level(&self) -> &LevelSpatialData {
        match self.levels_spatial_data.get(&self.current_level) {
            Some(level) => level,
            None => panic!("should never happen, world: {:?}", self),
        }
    }

    fn previous_spatial_level(&self) -> &LevelSpatialData {
        match self.levels_spatial_data.get(
            &self
                .previous_level
                .expect("should never call this when none"),
        ) {
            Some(level) => level,
            None => panic!("should never happen, world: {:?}", self),
        }
    }

    pub fn world_index(&self) -> usize {
        self.current_level.0
    }

    pub fn level_index(&self) -> usize {
        self.current_level.1
    }

    pub fn grid_matrix(&self) -> &Vec<Vec<u8>> {
        &self.current_spatial_level().grid_matrix
    }

    pub fn collider_polygons(&self) -> &Vec<Vec<Vec2>> {
        &self.current_spatial_level().collider_polygons
    }

    pub fn cached_level_data(&self) -> Option<CachedLevelData> {
        self.current_spatial_level().cached_data.clone()
    }

    pub fn cached_previous_level_data(&self) -> Option<CachedLevelData> {
        self.previous_level?;
        self.previous_spatial_level().cached_data.clone()
    }

    pub fn update_cached_level_data(&mut self, new_cached_data: CachedLevelData) {
        if self.previous_level.is_none() {
            return;
        }

        match self
            .levels_spatial_data
            .get_mut(&self.previous_level.expect("should never call this on none"))
        {
            Some(level) => level.cached_data = Some(new_cached_data),
            None => panic!("should never happen, world: {:?}", self),
        }
    }

    pub fn cached_player(&self) -> Option<CachedPlayer> {
        self.cached_player.clone()
    }

    pub fn set_cached_player(&mut self, cached_player: CachedPlayer) {
        self.cached_player = Some(cached_player);
    }

    pub fn level_dimensions(&self) -> UVec2 {
        let level = self.current_spatial_level();
        UVec2::new(
            level.grid_matrix.len() as u32,
            level.grid_matrix[0].len() as u32,
        )
    }

    pub fn pixel_coords_to_translation(&self, px_coords: IVec2) -> Vec2 {
        Vec2::new(
            px_coords.x as f32,
            self.level_dimensions().y as f32 * TILE_SIZE - px_coords.y as f32,
        )
    }

    pub fn transition_level(&mut self, direction: LevelChangeDirection) {
        assert_ne!(direction, LevelChangeDirection::None);

        match self.current_spatial_level().neighbours[direction.to_usize()] {
            Some(neighbour) => {
                self.previous_level = Some(self.current_level);
                self.current_level = (self.current_level.0, neighbour.0);
                self.level_transition_offset = IVec2::new(neighbour.1, neighbour.2);
                self.level_transition_direction = direction;
            }
            None => {
                panic!("trying to transition to neighbour that does not exist, your map is wrong!, current: {:?}, next: {:?}, dir: {:?}", self.current_level, self.current_spatial_level().neighbours, direction)
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
                cached_data: None,
            },
        );
    }

    commands.insert_resource(WorldSpatialData {
        levels_spatial_data,
        current_level: (0, 0),
        previous_level: None,
        level_transition_offset: IVec2::default(),
        level_transition_direction: LevelChangeDirection::North,
        cached_player: None,
    });
}

fn update_pf_source_target_positions(
    q_transforms: Query<&GlobalTransform>,
    mut q_pf_sources: Query<&mut PathfindingSource>,
) {
    for mut pf_source in &mut q_pf_sources {
        if let Some(target) = pf_source.target {
            if let Ok(transform) = q_transforms.get(target) {
                pf_source.target_pos = transform.translation().truncate();
            }
        }
    }
}
