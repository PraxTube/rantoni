use std::f32::consts::FRAC_1_SQRT_2;

use bevy::{color::palettes::css::RED, prelude::*};
use bevy_rancic::prelude::DebugState;
use bevy_rapier2d::prelude::*;

use crate::{
    dude::DudeState,
    world::{a_star, collisions::WORLD_GROUP, PathfindingSource, WorldSpatialData},
};

use super::{
    crowd::EnemyCrowd, spawn::COLLIDER_RADIUS, state::EnemyStateSystemSet, Enemy, MOVE_SPEED,
    STALK_SPEED,
};

const LINE_OF_SIGHT_COLLISION_GROUPS: CollisionGroups =
    CollisionGroups::new(WORLD_GROUP, WORLD_GROUP);

const ROT_MATRIX_LEFT: [[f32; 2]; 2] = [
    [FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
    [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
];
const ROT_MATRIX_RIGHT: [[f32; 2]; 2] = [
    [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
    [-FRAC_1_SQRT_2, FRAC_1_SQRT_2],
];

fn move_enemies(
    enemy_crowd: Res<EnemyCrowd>,
    mut q_enemies: Query<(Entity, &mut Velocity, &Enemy)>,
) {
    for (entity, mut velocity, enemy) in &mut q_enemies {
        let Some(target) = enemy.target else {
            continue;
        };

        let speed_mult = if let Some(index) = enemy_crowd
            .target_distances
            .get(&target)
            .and_then(|t_distances| t_distances.iter().position(|t| t.entity == entity))
        {
            let index = index.min(4);
            0.5 + 0.5 * (4 - index) as f32 / 4.0
        } else {
            1.0
        };

        match enemy.state_machine.state() {
            DudeState::Running => {
                velocity.linvel = enemy.move_direction * speed_mult * MOVE_SPEED;
            }
            DudeState::Staggering => {
                if !enemy.state_machine.stagger_state().is_recovering() {
                    velocity.linvel = enemy.state_machine.stagger_linvel();
                }
            }
            DudeState::Stalking => {
                velocity.linvel = enemy.move_direction * speed_mult * STALK_SPEED;
            }
            DudeState::Attacking => {
                velocity.linvel = enemy.state_machine.attack_direction() * 100.0;
            }
            _ => {}
        }
    }
}

fn rotate_vec(v: Vec2, rot_matrix: [[f32; 2]; 2]) -> Vec2 {
    Vec2::new(
        rot_matrix[0][0] * v.x + rot_matrix[0][1] * v.y,
        rot_matrix[1][0] * v.x + rot_matrix[1][1] * v.y,
    )
}

/// Check whether there is a clear line of sight between two points in the nav mash.
///
/// The definition of "clear line of sight" is that there is not collision with any
/// (WORLD_GROUP, WORLD_GROUP) for membership and filter respectively.
/// If the starting position is already inside of a collider with a WORLD_GROUP
/// then this is likely to fail.
///
/// It's meant to be only used for enemies -> player as of now, maybe it would also work for
/// enemy -> enemy.
fn clear_line_of_sight(
    gizmos: &mut Gizmos,
    rapier_context: &RapierContext,
    debug_state: &DebugState,
    pf_source: &mut PathfindingSource,
    pf_source_pos: Vec2,
    target: Entity,
    target_pos: Vec2,
) -> bool {
    let dir = (target_pos - pf_source_pos).normalize_or_zero() * COLLIDER_RADIUS;
    for offset in [dir.perp(), -dir.perp()] {
        if let Some((entity, _)) = rapier_context.cast_ray(
            pf_source_pos + offset,
            target_pos - pf_source_pos,
            1.0,
            false,
            QueryFilter::new().groups(LINE_OF_SIGHT_COLLISION_GROUPS),
        ) {
            if entity != target {
                return false;
            }
        }
    }

    if debug_state.0 {
        for offset in [dir.perp(), -dir.perp()] {
            gizmos.line_2d(pf_source_pos + offset, target_pos + offset, RED);
        }
    }

    pf_source.path = None;
    true
}

fn target_pos_from_path(
    map_polygon_data: &WorldSpatialData,
    pf_source: &mut PathfindingSource,
    pf_source_pos: Vec2,
    target_pos: Vec2,
) -> Vec2 {
    let path = a_star(
        pf_source_pos,
        target_pos,
        map_polygon_data.grid_matrix(),
        &pf_source.path,
    );
    pf_source.path = Some(path.clone());

    if path.is_empty() {
        return target_pos;
    }
    if path.len() == 1 {
        return path[0];
    }

    let path_dir = path[1] - path[0];
    let dir = pf_source_pos - path[0];

    let is_past_current_tile = dir.perp_dot(rotate_vec(path_dir, ROT_MATRIX_LEFT)) >= 0.0
        && dir.perp_dot(rotate_vec(path_dir, ROT_MATRIX_RIGHT)) <= 0.0;

    let distance_to_current_tile = pf_source_pos.distance_squared(path[0]);

    // Enemy is already past `path[0]`, so skip to the next point on path.
    if is_past_current_tile
        || distance_to_current_tile
        // TODO: figure out a good value here, also don't use magic numbers, make const or
        // something.
        // Okay well shit `https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/`
        < 10.0
    {
        path[1]
    } else {
        path[0]
    }
}

fn update_target_positions(
    mut gizmos: Gizmos,
    rapier_context: Res<RapierContext>,
    debug_state: Res<DebugState>,
    map_polygon_data: Res<WorldSpatialData>,
    mut q_enemies: Query<&mut Enemy>,
    mut q_pathfinding_sources: Query<(&GlobalTransform, &mut PathfindingSource)>,
) {
    for (pf_source_transform, mut pf_source) in &mut q_pathfinding_sources {
        let Ok(mut enemy) = q_enemies.get_mut(pf_source.root_entity) else {
            continue;
        };
        if enemy.target.is_none() {
            continue;
        }
        let Some(pf_target_entity) = pf_source.target else {
            continue;
        };

        let pf_source_pos = pf_source_transform.translation().truncate();
        let target_pos = pf_source.target_pos;

        let pos = if clear_line_of_sight(
            &mut gizmos,
            &rapier_context,
            &debug_state,
            &mut pf_source,
            pf_source_pos,
            pf_target_entity,
            target_pos,
        ) {
            target_pos
        } else {
            target_pos_from_path(&map_polygon_data, &mut pf_source, pf_source_pos, target_pos)
        };

        enemy.move_target_pos = pos;
    }
}

fn update_move_directions(
    mut q_enemies: Query<&mut Enemy>,
    q_pathfinding_sources: Query<(&GlobalTransform, &PathfindingSource)>,
) {
    for (transform, pf_source) in &q_pathfinding_sources {
        let Ok(mut enemy) = q_enemies.get_mut(pf_source.root_entity) else {
            continue;
        };
        if enemy.target.is_none() {
            continue;
        }

        enemy.move_direction =
            (enemy.move_target_pos - transform.translation().truncate()).normalize_or_zero();
    }
}

pub struct EnemyMovementPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_target_positions.run_if(resource_exists::<WorldSpatialData>),
                update_move_directions,
                move_enemies,
            )
                .chain()
                .after(EnemyStateSystemSet),
        );
    }
}
