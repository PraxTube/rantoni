mod pathfinding;

use bevy_rancic::prelude::{DebugState, ToggleDebugStateEvent};
pub use pathfinding::a_star;

use std::{fs, str::from_utf8};

use bevy::{color::palettes::css::*, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_lyon::prelude::*;

use generate_world_collisions::{deserialize_polygons, MAP_POLYGON_DATA, TILE_SIZE};

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::indices(1, 1))
            .add_systems(
                OnEnter(GameState::Gaming),
                (spawn_ldtk_world, insert_polygon_data),
            )
            .add_systems(
                Update,
                (
                    spawn_navmesh_debug_shapes.run_if(resource_added::<MapPolygonData>),
                    debug_enemy_pathfinding.run_if(resource_exists::<MapPolygonData>),
                    toggle_navmesh_polygons_visibility.run_if(on_event::<ToggleDebugStateEvent>()),
                ),
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

#[derive(Resource)]
pub struct MapPolygonData {
    pub grid_matrix: Vec<Vec<u8>>,
    pub collider_polygons: Vec<Vec<Vec2>>,
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
    let (grid_matrix, collider_polygons) = deserialize_polygons(serialized_data);

    commands.insert_resource(MapPolygonData {
        grid_matrix,
        collider_polygons,
    });
}

fn spawn_navmesh_debug_shapes(mut commands: Commands, map_polygon_data: Res<MapPolygonData>) {
    for i in 0..map_polygon_data.grid_matrix.len() {
        for j in 0..map_polygon_data.grid_matrix[i].len() {
            if map_polygon_data.grid_matrix[i][j] == 0 {
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
    map_polygon_data: Res<MapPolygonData>,
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

        let mut path = a_star(start, end, &map_polygon_data.grid_matrix, &None);

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
