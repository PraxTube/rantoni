use bevy::{
    color::palettes::css::{LIGHT_GREEN, PURPLE, WHITE},
    prelude::*,
};
use generate_world_collisions::TILE_SIZE;

use crate::{player::input::GlobalInput, world::DebugState, GameAssets};

use super::{LevelChanged, PathfindingSource, PathfindingTarget, WorldSpatialData};

const GRID_DEBUG_TEXT_Z_LEVEL: f32 = 100.0;

#[derive(Component)]
struct GridDebugVisual;

fn debug_enemy_pathfinding(
    mut gizmos: Gizmos,
    debug_state: Res<DebugState>,
    q_targets: Query<&GlobalTransform, With<PathfindingTarget>>,
    q_sources: Query<(&GlobalTransform, &PathfindingSource), Without<PathfindingTarget>>,
) {
    if !debug_state.active {
        return;
    }

    for (source_transform, pf_source) in &q_sources {
        let Some(target) = pf_source.target else {
            continue;
        };
        let Some(mut path) = pf_source.path.clone() else {
            continue;
        };
        let Ok(goal_transform) = q_targets.get(target) else {
            continue;
        };
        path.insert(0, source_transform.translation().truncate());
        path.push(goal_transform.translation().truncate());

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

fn spawn_grid_debug_visuals(
    commands: &mut Commands,
    assets: &GameAssets,
    world_data: &WorldSpatialData,
) {
    let grid_matrix = world_data.grid_matrix();
    for i in 0..grid_matrix.len() {
        for j in 0..grid_matrix[i].len() {
            commands.spawn((
                GridDebugVisual,
                Text2dBundle {
                    text: Text::from_section(
                        format!("{}", grid_matrix[i][j]),
                        TextStyle {
                            font: assets.pixel_font.clone(),
                            font_size: 30.0,
                            color: WHITE.into(),
                        },
                    ),
                    transform: Transform::from_xyz(
                        i as f32 * TILE_SIZE,
                        j as f32 * TILE_SIZE,
                        GRID_DEBUG_TEXT_Z_LEVEL,
                    ),
                    ..default()
                },
            ));

            if grid_matrix[i][j] == 0 {
                continue;
            }

            let alpha = if i % 2 == j % 2 { 0.2 } else { 0.35 };

            commands.spawn((
                GridDebugVisual,
                SpriteBundle {
                    sprite: Sprite {
                        color: PURPLE.with_alpha(alpha).into(),
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        i as f32 * TILE_SIZE,
                        j as f32 * TILE_SIZE,
                        GRID_DEBUG_TEXT_Z_LEVEL - 1.0,
                    ),
                    ..default()
                },
            ));
        }
    }
}

fn despawn_grid_debug_visuals(
    commands: &mut Commands,
    q_grid_debug_visuals: &Query<Entity, With<GridDebugVisual>>,
) {
    for entity in q_grid_debug_visuals {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_grid_debug_visuals(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut debug_state: ResMut<DebugState>,
    global_input: Res<GlobalInput>,
    world_data: Res<WorldSpatialData>,
    q_grid_debug_visuals: Query<Entity, With<GridDebugVisual>>,
) {
    if global_input.toggle_debug {
        if debug_state.active && debug_state.grid_visuals_active {
            spawn_grid_debug_visuals(&mut commands, &assets, &world_data);
        } else if !debug_state.active {
            despawn_grid_debug_visuals(&mut commands, &q_grid_debug_visuals);
        }
        return;
    }

    if !debug_state.active {
        return;
    }
    if !global_input.toggle_grid_debug_visuals {
        return;
    }

    if debug_state.grid_visuals_active {
        debug_state.grid_visuals_active = false;
        despawn_grid_debug_visuals(&mut commands, &q_grid_debug_visuals);
    } else {
        debug_state.grid_visuals_active = true;
        spawn_grid_debug_visuals(&mut commands, &assets, &world_data);
    }
}

fn despawn_grid_debug_visuals_on_level_transition(
    mut commands: Commands,
    q_grid_debug_visuals: Query<Entity, With<GridDebugVisual>>,
) {
    despawn_grid_debug_visuals(&mut commands, &q_grid_debug_visuals);
}

fn spawn_grid_debug_visuals_on_level_transition(
    mut commands: Commands,
    assets: Res<GameAssets>,
    debug_state: Res<DebugState>,
    world_data: Res<WorldSpatialData>,
) {
    if !debug_state.active {
        return;
    }
    if !debug_state.grid_visuals_active {
        return;
    }

    spawn_grid_debug_visuals(&mut commands, &assets, &world_data);
}

pub struct MapDebugPlugin;

impl Plugin for MapDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (debug_enemy_pathfinding, toggle_grid_debug_visuals)
                .run_if(resource_exists::<WorldSpatialData>),
        )
        .add_systems(
            Update,
            (
                despawn_grid_debug_visuals_on_level_transition,
                spawn_grid_debug_visuals_on_level_transition,
            )
                .chain()
                .run_if(resource_exists::<WorldSpatialData>.and_then(on_event::<LevelChanged>())),
        );
    }
}
