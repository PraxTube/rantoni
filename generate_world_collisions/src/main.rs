#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod decomposition;
mod graph;

use std::time::Duration;

use bevy::color::palettes::css::VIOLET;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::time::common_conditions::once_after_delay;
use bevy::window::{PresentMode, Window, WindowMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use decomposition::decompose_poly;
use graph::{disjoint_graphs, vertices_and_indices};

const TILE_SIZE: f32 = 16.0;
const LDTK_FILE: &str = "map/map.ldtk";

#[derive(Resource)]
struct Grid {
    size: IVec2,
    positions: Vec<IVec2>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            // TODO: We need grid size + 1, presumably because we need to test one direction to the
            // right and top and diagonal for each grid position, so for the top right corner we
            // need to check the top right corner + 1, which are always supposed to be 0 anyways,
            // so we can just instantiate them as 0's, as they will always be 0 when the grid is at
            // most size big.
            size: IVec2::new(17, 17),
            positions: Vec::new(),
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(AssetPlugin {
                file_path: "../assets/".to_string(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Fifo,
                    mode: WindowMode::Windowed,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .build(),))
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: true,
                ..default()
            },
        ))
        .insert_resource(LevelSelection::index(0))
        .init_resource::<Grid>()
        .add_systems(Startup, setup)
        .add_systems(Update, (add_cells,))
        .add_systems(
            Update,
            spawn_colliders.run_if(once_after_delay(Duration::from_secs_f32(0.5))),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = true;
    config.line_width = 5.0;

    let mut cam = Camera2dBundle::default();
    cam.projection.scaling_mode = ScalingMode::FixedVertical(300.0);
    cam.transform = Transform::from_translation(Vec3::new(128.0, 128.0, 0.0));
    commands.spawn(cam);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(LDTK_FILE),
        ..Default::default()
    });
}

fn add_cells(mut grid: ResMut<Grid>, q_grid_coords: Query<&GridCoords, Added<IntGridCell>>) {
    for grid_coords in &q_grid_coords {
        grid.positions
            .push(IVec2::new(grid_coords.x, grid_coords.y));
    }
}

fn spawn_colliders(mut commands: Commands, grid: Res<Grid>) {
    for graph in disjoint_graphs(&grid) {
        let grid = Grid {
            size: grid.size,
            positions: graph,
        };
        let (vertices, _) = vertices_and_indices(&grid);

        let polygons = decompose_poly(&mut vertices.clone());
        for poly in &polygons {
            commands.spawn((
                Collider::compound(vec![(
                    Vec2::default(),
                    0.0,
                    Collider::convex_hull(poly).unwrap(),
                )]),
                ColliderDebugColor(VIOLET.into()),
                SpatialBundle::default(),
            ));
        }
    }
}

#[test]
fn polygon_at_edge() {
    let grid = Grid {
        size: IVec2::new(3, 4),
        positions: vec![IVec2::new(0, 1), IVec2::new(0, 2)],
    };

    let (mut vertices, _) = vertices_and_indices(&grid);

    let expeced_vertices = vec![
        Vec2::new(0.0, 8.0),
        Vec2::new(8.0, 16.0),
        Vec2::new(8.0, 32.0),
        Vec2::new(0.0, 40.0),
        Vec2::new(0.0, 32.0),
        Vec2::new(0.0, 16.0),
    ];
    let expected_polygon = vec![
        Vec2::new(0.0, 8.0),
        Vec2::new(8.0, 16.0),
        Vec2::new(8.0, 32.0),
        Vec2::new(0.0, 40.0),
        Vec2::new(0.0, 32.0),
        Vec2::new(0.0, 16.0),
    ];

    let polygons = decompose_poly(&mut vertices);
    assert!(polygons.len() == 1);

    for polygon in &polygons {
        assert_eq!(polygon, &expected_polygon);
    }

    assert_eq!(expeced_vertices, vertices);
}

#[test]
fn polygon_with_hole_decomposition() {
    let mut vertices = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 0.0),
        Vec2::new(10.0, 5.0),
        Vec2::new(6.0, 5.0),
        Vec2::new(6.0, 4.0),
        Vec2::new(4.0, 4.0),
        Vec2::new(4.0, 5.0),
        Vec2::new(6.0, 5.0),
        Vec2::new(10.0, 5.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 10.0),
    ];

    let polygons = decompose_poly(&mut vertices);
    assert_eq!(polygons.len(), 4);
}
