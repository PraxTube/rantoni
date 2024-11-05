#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::fs;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::time::common_conditions::once_after_delay;
use bevy::window::{PresentMode, Window, WindowMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use generate_world_collisions::{
    decompose_poly, merge_convex_polygons, serialize_polygons, Grid, LDTK_FILE, MAP_POLYGON_DATA,
};

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
    let mut polygons = decompose_poly(&Grid {
        size: grid.size,
        positions: grid.positions.clone(),
        is_navmesh: true,
    });
    merge_convex_polygons(&mut polygons);
    polygons
}

fn compute_collier_shapes(grid: &Grid) -> Vec<Vec<Vec2>> {
    let mut polygons = decompose_poly(&Grid {
        size: grid.size,
        positions: grid.positions.clone(),
        is_navmesh: false,
    });
    merge_convex_polygons(&mut polygons);
    polygons
}

fn compute_and_save_shapes(grid: Res<Grid>, mut app_exit_events: EventWriter<AppExit>) {
    let contents = format!(
        "{}\n{}",
        serialize_polygons(&compute_navmesh_shapes(&grid)),
        serialize_polygons(&compute_collier_shapes(&grid))
    );
    fs::write(MAP_POLYGON_DATA, contents).unwrap();
    app_exit_events.send(AppExit::Success);
}
