use std::{fs, str::from_utf8};

use bevy::{color::palettes::css::*, prelude::*, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy_rancic::prelude::ToggleDebugStateEvent;
use generate_world_collisions::{deserialize_polygons, is_ccw, ATOL, MAP_POLYGON_DATA};

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
                    testy.run_if(resource_exists::<MapPolygonData>),
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

    let adjacency_graph = get_graph(&navmesh_polygons);

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

fn testy(
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
            &get_graph(&map_polygon_data.navmesh_polygons),
        );

        path.insert(0, (69, start));
        path.push((420, end));

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

fn adjacency_edge(poly_a: &Vec<Vec2>, poly_b: &Vec<Vec2>) -> Option<(Vec2, Vec2)> {
    let mut first_shared_vertex = None;
    for v in poly_a {
        for u in poly_b {
            if v.abs_diff_eq(*u, ATOL) {
                match first_shared_vertex {
                    Some(shared_vertex) => {
                        return {
                            assert_ne!(
                                shared_vertex, *u,
                                "poly a: {:?}, poly b: {:?}",
                                poly_a, poly_b
                            );
                            Some((shared_vertex, *u))
                        }
                    }
                    None => first_shared_vertex = Some(*u),
                };
            }
        }
    }
    None
}

fn get_graph(navmesh_polygons: &Vec<Vec<Vec2>>) -> Vec<Vec<(usize, (Vec2, Vec2))>> {
    let mut graph = Vec::new();

    // Find adjacency polygons
    for (i, poly) in navmesh_polygons.iter().enumerate() {
        graph.push(Vec::new());
        for (j, other_poly) in navmesh_polygons.iter().enumerate() {
            if i == j {
                continue;
            }

            let Some(edge) = adjacency_edge(poly, other_poly) else {
                continue;
            };

            // j is a neigbhour of i, with `edge` the shared edge between them
            graph[i].push((j, (edge.0, edge.1)));
        }
    }
    graph
}

fn area(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    ((b.x - a.x) * (c.y - a.y)) - ((c.x - a.x) * (b.y - a.y))
}

fn left(a: Vec2, b: Vec2, c: Vec2) -> bool {
    area(a, b, c) > 0.0
}

fn pos_in_poly(poly: &Vec<Vec2>, v: Vec2) -> bool {
    assert!(is_ccw(poly));
    assert!(poly.len() > 2);
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        // Collinear
        if area(a, b, v) == 0.0 {
            return true;
        }
        // Because poly is counter-clockwise oriented, the point lies outside the poly.
        if !left(a, b, v) {
            return false;
        }
    }
    true
}

// Given v: Vec2, determine in which polygon it lies.
// Panics if it doesn't lie in any polygon.
fn pos_to_polygon(polygons: &Vec<Vec<Vec2>>, v: Vec2) -> Option<usize> {
    for (i, poly) in polygons.iter().enumerate() {
        // Point is left for all edges of this polygon, so it must be inside
        // `https://inginious.org/course/competitive-programming/geometry-pointinconvex#`
        if pos_in_poly(poly, v) {
            return Some(i);
        }
    }
    error!("v: {} is not in any navmesh node!", v);
    None
}

fn reconstruct_path(
    parents: &Vec<Option<(usize, Vec2)>>,
    mut current_node: (usize, Vec2),
) -> Vec<(usize, Vec2)> {
    let mut path = Vec::new();
    while let Some(parent) = parents[current_node.0] {
        current_node = parent;
        path.push(current_node);
    }
    path.reverse();
    path
}

fn closest_point_on_edge(p: Vec2, e: (Vec2, Vec2)) -> Vec2 {
    assert!(!e.0.abs_diff_eq(e.1, ATOL));
    if (e.0.x - e.1.x).abs() < ATOL {
        return Vec2::new(e.0.x, p.y);
    }
    if (e.0.y - e.1.y).abs() < ATOL {
        return Vec2::new(p.x, e.0.y);
    }

    assert!((e.0.x - e.1.x).abs() > ATOL);
    assert!((e.0.y - e.1.y).abs() > ATOL);

    let m1 = (e.1.y - e.0.y) / (e.1.x - e.0.x);
    let m2 = -1.0 / m1;

    // Calculate projected point
    let x = (m1 * e.0.x - m2 * p.x + p.y - e.0.y) / (m1 - m2);
    let y = m2 * (x - p.x) + p.y;

    let edge_dir = e.1 - e.0;
    let projected_dir = Vec2::new(x, y) - e.0;
    // Clamp projected point to edge
    if x.abs() < ATOL {
        if projected_dir.y < 0.0 && edge_dir.y > 0.0 {
            return e.0;
        }
        if projected_dir.y > edge_dir.y {
            return e.1;
        }
    } else {
        if projected_dir.x < 0.0 && edge_dir.x > 0.0 {
            return e.0;
        }
        if projected_dir.x > edge_dir.x {
            return e.1;
        }
    }

    Vec2::new(x, y)
}

fn key_of_smallest_value(h: &HashMap<usize, f32>) -> usize {
    let mut smallest_value = f32::MAX;
    let mut current_key = None;
    for (key, value) in h {
        if *value < smallest_value {
            smallest_value = *value;
            current_key = Some(key)
        }
    }
    *current_key.expect("Something went very wrong with you smallest value in hashmap fn")
}

fn middle_point(a: Vec2, b: Vec2) -> Vec2 {
    a + (b - a) / 2.0
}

pub fn a_star(
    start: Vec2,
    goal: Vec2,
    polygons: &Vec<Vec<Vec2>>,
    graph: &Vec<Vec<(usize, (Vec2, Vec2))>>,
) -> Vec<(usize, Vec2)> {
    fn h(v: Vec2, end: Vec2) -> f32 {
        v.distance_squared(end)
    }

    fn d(p: Vec2, e: (Vec2, Vec2)) -> f32 {
        p.distance_squared(middle_point(e.0, e.1))
    }

    let Some(start_polygon) = pos_to_polygon(polygons, start) else {
        return Vec::new();
    };
    let Some(goal_polygon) = pos_to_polygon(polygons, goal) else {
        return Vec::new();
    };

    // Given points are already in the same polygon, trivial case.
    if start_polygon == goal_polygon {
        return Vec::new();
    }

    let mut nodes_to_explore = HashMap::new();
    nodes_to_explore.insert(start_polygon, 0.0);

    let mut parents = vec![None; polygons.len()];

    let mut global_scores = vec![f32::MAX; polygons.len()];
    global_scores[start_polygon] = 0.0;

    let mut local_scores = vec![f32::MAX; polygons.len()];
    local_scores[start_polygon] = h(start, goal);

    let mut current_location = start;

    while !nodes_to_explore.is_empty() {
        let current_node = key_of_smallest_value(&nodes_to_explore);

        if current_node == goal_polygon {
            return reconstruct_path(&parents, (current_node, Vec2::ZERO));
        }

        nodes_to_explore
            .remove(&current_node)
            .expect("should not be empty, something is fishy with the while loop");

        for neigbhour in &graph[current_node] {
            assert_ne!(current_node, neigbhour.0);

            let tentative_score = global_scores[current_node] + d(current_location, neigbhour.1);
            if tentative_score < global_scores[neigbhour.0] {
                current_location = middle_point(neigbhour.1 .0, neigbhour.1 .1);
                parents[neigbhour.0] = Some((current_node, current_location));
                global_scores[neigbhour.0] = tentative_score;
                local_scores[neigbhour.0] = tentative_score + h(current_location, goal);

                if !nodes_to_explore.contains_key(&neigbhour.0) {
                    nodes_to_explore.insert(neigbhour.0, local_scores[neigbhour.0]);
                }
            }
        }
    }
    panic!("There is no path between, start: {}, end: {}\nShould never happen, this most likely means you have some navmesh islands which is not supported, as they don't make much sense.", start, goal);
}

#[test]
fn test_closest_point_to_edge() {
    let points_and_edges = [
        (Vec2::ONE, (Vec2::new(10.0, 10.0), Vec2::new(50.0, 20.0))),
        (
            Vec2::new(0.0, 50.0),
            (Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0)),
        ),
    ];
    let expected_points = [Vec2::new(10.0, 10.0), Vec2::new(25.0, 25.0)];

    assert_eq!(points_and_edges.len(), expected_points.len());

    for i in 0..points_and_edges.len() {
        let p = closest_point_on_edge(points_and_edges[i].0, points_and_edges[i].1);
        assert_eq!(p, expected_points[i]);
    }
}

#[test]
fn test_pos_to_polygon() {
    let polygons = vec![vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 10.0),
    ]];
    assert_eq!(pos_to_polygon(&polygons, Vec2::new(100.0, 10.0)), None);
    assert_eq!(pos_to_polygon(&polygons, Vec2::new(10.0, 10.0)), Some(0));
    assert_eq!(pos_to_polygon(&polygons, Vec2::new(5.0, 10.0)), Some(0));
    assert_eq!(pos_to_polygon(&polygons, Vec2::new(10.0, 1.0)), None);
}

#[test]
#[should_panic(expected = "assertion failed: is_ccw(poly)")]
fn test_panic_when_polygon_not_ccw() {
    let polygons = vec![vec![
        Vec2::new(0.0, 10.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 0.0),
    ]];
    assert_eq!(pos_to_polygon(&polygons, Vec2::new(100.0, 10.0)), None);
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
    let graph = get_graph(&polygons);
    let path = a_star(start, goal, &polygons, &graph);

    assert_eq!(path.len(), 2);

    let expected_indices = [0, 1, 2];
    let mut expected_positions = vec![closest_point_on_edge(start, graph[0][0].1.clone())];

    for i in 1..path.len() {
        expected_positions.push(closest_point_on_edge(
            expected_positions[i - 1],
            graph[i][graph[i].len() - 1].1.clone(),
        ));
    }

    for i in 0..path.len() {
        assert_eq!(path[i].0, expected_indices[i]);
        assert_eq!(path[i].1, expected_positions[i], "index: {}", i);
    }
}
