mod pathfinding;

use bevy_rancic::prelude::{DebugState, ToggleDebugStateEvent};
pub use pathfinding::a_star;

use std::{fs, str::from_utf8, time::Duration};

use bevy::{
    color::palettes::css::*, prelude::*, time::common_conditions::on_timer, utils::HashMap,
};
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_lyon::prelude::*;

use generate_world_collisions::{deserialize_polygons, MAP_POLYGON_DATA, TILE_SIZE};

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::indices(0, 0))
            .add_event::<LevelChanged>()
            .add_systems(OnExit(GameState::AssetLoading), spawn_ldtk_world)
            .add_systems(OnEnter(GameState::Gaming), insert_polygon_data)
            .add_systems(
                Update,
                (
                    spawn_navmesh_debug_shapes.run_if(resource_added::<WorldSpatialData>),
                    debug_enemy_pathfinding.run_if(resource_exists::<WorldSpatialData>),
                    toggle_navmesh_polygons_visibility.run_if(on_event::<ToggleDebugStateEvent>()),
                ),
            )
            .add_systems(
                Update,
                change_level_on_start.run_if(on_timer(Duration::from_secs_f32(2.0))),
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

#[derive(Component)]
struct DebugNavmeshPolygon;

#[derive(Resource, Debug)]
pub struct WorldSpatialData {
    pub levels_spatial_data: HashMap<(usize, usize), LevelSpatialData>,
    current_level: (usize, usize),
}

#[derive(Debug)]
pub struct LevelSpatialData {
    pub grid_matrix: Vec<Vec<u8>>,
    pub collider_polygons: Vec<Vec<Vec2>>,
    pub neighbours: [Option<usize>; 4],
}

#[derive(Event)]
pub struct LevelChanged;

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
}

fn spawn_ldtk_world(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.map.clone(),
        transform: Transform::from_translation(Vec3::Z * Z_LEVEL_BACKGROUND),
        ..default()
    });
}

fn insert_polygon_data(mut commands: Commands) {
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

fn spawn_navmesh_debug_shapes(mut commands: Commands, world_data: Res<WorldSpatialData>) {
    for i in 0..world_data.grid_matrix().len() {
        for j in 0..world_data.grid_matrix()[i].len() {
            if world_data.grid_matrix()[i][j] == 0 {
                continue;
            }

            let color = if i % 2 == 0 {
                if j % 2 == 0 {
                    RED
                } else {
                    DARK_RED
                }
            } else if j % 2 == 0 {
                DARK_RED
            } else {
                RED
            };

            commands.spawn((
                DebugNavmeshPolygon,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: Vec2::new(TILE_SIZE, TILE_SIZE),
                        origin: RectangleOrigin::Center,
                    }),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(
                            i as f32 * TILE_SIZE,
                            j as f32 * TILE_SIZE,
                            Z_LEVEL_BACKGROUND + 10.0,
                        ),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    ..default()
                },
                Fill::color(color.with_alpha(0.5)),
            ));
        }
    }
}

fn toggle_navmesh_polygons_visibility(
    mut q_navmesh_visibilities: Query<&mut Visibility, With<DebugNavmeshPolygon>>,
) {
    for mut visibility in &mut q_navmesh_visibilities {
        let new_visibility = match *visibility {
            Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Inherited,
        };
        *visibility = new_visibility;
    }
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

fn change_level_on_start(mut ev_level_changed: EventWriter<LevelChanged>) {
    ev_level_changed.send(LevelChanged);
}
