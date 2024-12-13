#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_range_loop
)]

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
    decompose_poly, map_grid_matrix, merge_convex_polygons, serialize_collider_polygons,
    serialize_grid_matrix, Grid, DIAGONAL_CONCRETE, DIAGONAL_WALKABLE_INDEX, LDTK_FILE,
    MAP_POLYGON_DATA, PLAYER_LAYER_IDENTIFIER, SQUARE_CONCRETE_IDENTIFIER, STRAIGHT_WALKABLE_INDEX,
    TILE_SIZE,
};
use ldtk::WorldLayout;

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
        .insert_resource(LevelSelection::indices(0, 0))
        .add_systems(Startup, setup)
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
        ..Default::default()
    });
}

fn compute_collier_shapes(grid: &Grid) -> Vec<Vec<Vec2>> {
    let mut polygons = decompose_poly(grid);
    merge_convex_polygons(&mut polygons);
    polygons
}

fn grid_from_layer(width: usize, height: usize, layer: &LayerInstance, value: u8) -> Grid {
    assert_eq!(layer.grid_size as f32, TILE_SIZE);
    assert_eq!(layer.layer_instance_type, ldtk::Type::IntGrid);
    assert_eq!(layer.int_grid_csv.len(), height * width);

    let mut grid = Grid::new(width + 1, height + 1);
    for x in 0..width {
        for y in 0..height {
            if layer.int_grid_csv[y * width + x] == 0 {
                continue;
            }
            grid.set_grid_value(x, y, value);
        }
    }
    grid
}

fn grids_from_level(level: &ldtk::Level) -> Grid {
    let width = (level.px_wid as f32 / TILE_SIZE) as usize;
    let height = (level.px_hei as f32 / TILE_SIZE) as usize;

    let mut combined_grid = Grid::new(width + 1, height + 1);

    for layer in level
        .layer_instances
        .clone()
        .expect("you should never use 'separate levels' option")
    {
        assert_eq!(layer.grid_size as f32, TILE_SIZE);
        if layer.layer_instance_type != ldtk::Type::IntGrid {
            continue;
        }

        let grid = match layer.identifier.as_str() {
            SQUARE_CONCRETE_IDENTIFIER => {
                grid_from_layer(width, height, &layer, STRAIGHT_WALKABLE_INDEX)
            }
            DIAGONAL_CONCRETE => grid_from_layer(width, height, &layer, DIAGONAL_WALKABLE_INDEX),
            _ => continue,
        };

        info!("{}, {}", level.identifier, layer.identifier);
        combined_grid.or_grid(&grid);
    }
    combined_grid
}

fn level_neigbhours(world: &ldtk::World, level: &ldtk::Level) -> String {
    let mut neighbours = [None; 4];
    let mut dirs = Vec::new();

    for level_neighbour in &level.neighbours {
        // This neighbour is diagonal, meaning he is only connected to the most outer vertex, there
        // isn't even a shared edge. We don't allow for this type of neighbouring levels.
        if ["ne", "se", "sw", "nw"].contains(&&*level_neighbour.dir) {
            continue;
        }
        let mut index = None;

        for (i, level) in world.levels.iter().enumerate() {
            if level.iid == level_neighbour.level_iid {
                index = Some(i);
                break;
            }
        }

        let i = match &*level_neighbour.dir {
            "n" => 0,
            "e" => 1,
            "s" => 2,
            "w" => 3,
            _ => panic!(
                "this type of direction is not supported, {}",
                level_neighbour.dir
            ),
        };

        match index {
            Some(index) =>
                neighbours[i] = Some((
                    index,
                    level.world_x - world.levels[index].world_x,
                    (-level.world_y - level.px_hei) - (-world.levels[index].world_y - world.levels[index].px_hei),
                )),
                None => panic!("the world must always contain the level neighbours, perhaps can other worlds also contain the neighbours for some reason? Read the docs of bevy_ecs_ldtk more carefully"),
        }

        assert!(
            !dirs.contains(&level_neighbour.dir),
            "we don't allow more then 1 neighbour for each side of a level, this is because of technical reasons, I may adjust this in the future to allow more artistic freedom, but it would be nice if it would work out this way"
        );
        dirs.push(level_neighbour.dir.clone());
    }

    assert!(!neighbours.is_empty());
    neighbours
        .iter()
        .map(|u| match u {
            Some((index, x_offset, y_offset)) => format!("{},{},{}", index, x_offset, y_offset),
            None => "-".to_string(),
        })
        .collect::<Vec<String>>()
        .join(";")
}

fn sanity_checks(world_index: usize, level_index: usize, level: &ldtk::Level) {
    let layers = level
        .layer_instances
        .clone()
        .expect("you should never use 'separate levels' option");

    assert!(!layers.is_empty());

    for layer in layers {
        assert_eq!(layer.grid_size as f32, TILE_SIZE);

        if layer.identifier == PLAYER_LAYER_IDENTIFIER {
            assert_eq!(layer.layer_instance_type, ldtk::Type::Entities);
            if level_index == 0 {
                assert_eq!(
                    layer.entity_instances.len(),
                    1,
                    "Must have exactly one player present in first level, world: {}, level: {}",
                    world_index,
                    level_index
                );
            } else {
                assert!(
                    layer.entity_instances.is_empty(),
                    "There must not be any players in other levels, world: {}, level: {}",
                    world_index,
                    level_index
                );
            }
        }
    }
}

/// Make sure that the diagonals of the grid aren't mixed, meaning that there are no invalid 1's
/// and 2's mixing together. This makes sure that the pathfinding is correct, it doesn't however
/// check for visual correctness.
fn grid_validation_check(grid: &Grid) {
    fn validate_diagonals(grid: &Grid, x: usize, y: usize) {
        for (i, j) in [
            (x.max(1) - 1, y.max(1) - 1),
            ((x + 1).min(grid.width - 1), y.max(1) - 1),
            (x.max(1) - 1, (y + 1).min(grid.height - 1)),
            ((x + 1).min(grid.width - 1), (y + 1).min(grid.height - 1)),
        ] {
            if grid.grid[x][y] == STRAIGHT_WALKABLE_INDEX
                && grid.grid[i][j] == DIAGONAL_WALKABLE_INDEX
            {
                assert_ne!(
                    grid.grid[i][y], 0,
                    "grid with width: {}, height: {} at x: {}, y: {} with problematic diagonal: i: {}, j: {}",
                    grid.width, grid.height, x, y, i, y
                );
                assert_ne!(grid.grid[x][j], 0,
                    "grid with width: {}, height: {} at x: {}, y: {} with problematic diagonal: i: {}, j: {}",
                    grid.width, grid.height, x, y, x, j
                );
            }
        }
    }

    for i in 0..grid.width - 1 {
        for j in 0..grid.height - 1 {
            validate_diagonals(grid, i, j);
        }
    }
}

fn compute_and_save_shapes(
    asset_server: Res<AssetServer>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let project = ldtk_project_assets
        .get(&asset_server.load(LDTK_FILE))
        .expect("ldtk project should be loaded at this point, maybe time was not enough, is the project really big?");

    let mut contents = Vec::new();
    for (i, world) in project.worlds().iter().enumerate() {
        assert_eq!(world.world_layout, Some(WorldLayout::Free));
        for (j, level) in world.levels.iter().enumerate() {
            assert!(level.px_wid > 0);
            assert!(level.px_hei > 0);
            sanity_checks(i, j, &level);

            let neighbour_indices = level_neigbhours(world, level);
            let grid = grids_from_level(&level);
            grid_validation_check(&grid);

            contents.push(format!(
                "{},{}:{}@{}@{}",
                i,
                j,
                serialize_grid_matrix(&map_grid_matrix(&grid)),
                serialize_collider_polygons(&compute_collier_shapes(&grid)),
                neighbour_indices
            ));
        }
    }

    fs::write(MAP_POLYGON_DATA, contents.join("\n")).unwrap();
    app_exit_events.send(AppExit::Success);
}
