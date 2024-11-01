#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod decomposition;
mod graph;
mod serialization;

use std::time::Duration;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::time::common_conditions::once_after_delay;
use bevy::window::{PresentMode, Window, WindowMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use decomposition::decompose_poly;
use graph::{disjoint_graphs_colliders, disjoint_graphs_walkable, polygons};
use serialization::save_polygons;

// TODO: Adjust?
const TILE_SIZE: f32 = 16.0;
const LDTK_FILE: &str = "map/map.ldtk";

#[derive(Resource)]
struct Grid {
    size: IVec2,
    positions: Vec<IVec2>,
    is_navmesh: bool,
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
            is_navmesh: true,
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
            ShapePlugin,
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
            compute_and_save_shapes.run_if(once_after_delay(Duration::from_secs_f32(0.5))),
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
        transform: Transform::from_translation(Vec3::NEG_Z * 10.0),
        ..Default::default()
    });
}

fn add_cells(mut grid: ResMut<Grid>, q_grid_coords: Query<&GridCoords, Added<IntGridCell>>) {
    for grid_coords in &q_grid_coords {
        grid.positions
            .push(IVec2::new(grid_coords.x, grid_coords.y));
    }
}

fn compute_navmesh_shapes(grid: &Grid) -> Vec<Vec<Vec2>> {
    let mut navmesh_polygons = Vec::new();
    for graph in disjoint_graphs_walkable(&grid) {
        let grid = Grid {
            size: grid.size,
            positions: graph,
            is_navmesh: true,
        };
        let (outer_polygon, inner_polygons) = polygons(&grid);

        navmesh_polygons.append(&mut decompose_poly(&outer_polygon, &inner_polygons));
    }
    navmesh_polygons
}

fn compute_collier_shapes(grid: &Grid) -> Vec<Vec<Vec2>> {
    let mut collider_polygons = Vec::new();
    for graph in disjoint_graphs_colliders(grid) {
        let grid = Grid {
            size: grid.size,
            positions: graph,
            is_navmesh: false,
        };
        let (outer_polygon, inner_polygons) = polygons(&grid);

        collider_polygons.append(&mut decompose_poly(&outer_polygon, &inner_polygons));
    }
    collider_polygons
}

fn compute_and_save_shapes(grid: Res<Grid>, mut app_exit_events: EventWriter<AppExit>) {
    save_polygons(
        &compute_navmesh_shapes(&grid),
        &compute_collier_shapes(&grid),
    );
    app_exit_events.send(AppExit::Success);
}
