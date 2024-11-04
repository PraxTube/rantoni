mod pathfinding;

pub use pathfinding::a_star;

use std::{fs, str::from_utf8};

use bevy::{color::palettes::css::*, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy_rancic::prelude::ToggleDebugStateEvent;
use generate_world_collisions::{
    construct_adjacency_graph, deserialize_polygons, MAP_POLYGON_DATA,
};

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;

// const NAVMESH_FILL_COLORS: [Srgba; 10] = [
//     RED, AQUA, BLUE, GREEN, MAROON, NAVY, OLIVE, TEAL, YELLOW, PURPLE,
// ];

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(
                OnEnter(GameState::Gaming),
                (spawn_ldtk_world, insert_polygon_data),
            )
            .add_systems(
                Update,
                (
                    spawn_navmesh_debug_shapes.run_if(resource_added::<MapPolygonData>),
                    toggle_navmesh_polygons_visibility.run_if(on_event::<ToggleDebugStateEvent>()),
                    debug_enemy_pathfinding.run_if(resource_exists::<MapPolygonData>),
                ),
            );
    }
}

#[derive(Component)]
pub struct PathfindingSource;
#[derive(Component)]
pub struct PathfindingTarget;

#[derive(Component)]
struct DebugNavmeshPolygon;

#[derive(Resource)]
pub struct MapPolygonData {
    pub navmesh_polygons: Vec<Vec<Vec2>>,
    pub collider_polygons: Vec<Vec<Vec2>>,
    pub adjacency_graph: Vec<Vec<(usize, (Vec2, Vec2))>>,
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
    let (navmesh_polygons, collider_polygons) = deserialize_polygons(serialized_data);

    let adjacency_graph = construct_adjacency_graph(&navmesh_polygons);

    commands.insert_resource(MapPolygonData {
        navmesh_polygons,
        collider_polygons,
        adjacency_graph,
    });
}

fn spawn_navmesh_debug_shapes(mut commands: Commands, map_polygon_data: Res<MapPolygonData>) {
    for (i, poly) in map_polygon_data.navmesh_polygons.iter().enumerate() {
        commands.spawn((
            DebugNavmeshPolygon,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Polygon {
                    points: poly.clone(),
                    closed: true,
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, Z_LEVEL_BACKGROUND + 10.0),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::srgb(
                i as f32 / map_polygon_data.navmesh_polygons.len() as f32,
                0.0,
                0.0,
            )),
        ));
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
    map_polygon_data: Res<MapPolygonData>,
    q_player: Query<&GlobalTransform, With<PathfindingTarget>>,
    q_enemies: Query<&GlobalTransform, (With<PathfindingSource>, Without<PathfindingTarget>)>,
) {
    let Ok(player_transform) = q_player.get_single() else {
        return;
    };

    for enemy_transform in &q_enemies {
        let start = enemy_transform.translation().truncate();
        let end = player_transform.translation().truncate();

        let mut path = a_star(
            start,
            end,
            &map_polygon_data.navmesh_polygons,
            &map_polygon_data.adjacency_graph,
        );

        path.insert(0, (0, start));
        path.push((0, end));

        for i in 0..path.len() - 1 {
            let color = Srgba::new(
                LIGHT_GREEN.red,
                LIGHT_GREEN.green * i as f32 / (path.len() - 1) as f32,
                LIGHT_GREEN.blue,
                1.0,
            );
            gizmos.line_2d(path[i].1, path[i + 1].1, color);
        }
    }
}

#[test]
fn test_a_start_path() {
    let start = Vec2::new(2.0, 2.0);
    let goal = Vec2::new(7.0, 16.0);
    let polygons = vec![
        vec![Vec2::ZERO, Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0)],
        vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(20.0, 5.0),
        ],
        vec![
            Vec2::new(5.0, 20.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(20.0, 5.0),
        ],
    ];
    let graph = construct_adjacency_graph(&polygons);
    let path = a_star(start, goal, &polygons, &graph);

    assert_eq!(path.len(), 2);
}
