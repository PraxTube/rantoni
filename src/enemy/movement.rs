use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    dude::DudeState,
    world::{a_star, MapPolygonData, PathfindingSource, PathfindingTarget},
};

use super::{state::EnemyStateSystemSet, Enemy, MOVE_SPEED};

const ROT_MATRIX_LEFT: [[f32; 2]; 2] = [[0.7071, -0.7071], [0.7071, 0.7071]];
const ROT_MATRIX_RIGHT: [[f32; 2]; 2] = [[0.7071, 0.7071], [-0.7071, 0.7071]];

fn move_enemies(mut q_enemies: Query<(&mut Velocity, &Enemy)>) {
    for (mut velocity, enemy) in &mut q_enemies {
        match enemy.state_machine.state() {
            DudeState::Running => {
                velocity.linvel = enemy.move_direction * MOVE_SPEED;
            }
            DudeState::Staggering => {
                if !enemy.state_machine.stagger_state().is_recovering() {
                    velocity.linvel = enemy.state_machine.stagger_linvel();
                }
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

fn update_target_positions(
    map_polygon_data: Res<MapPolygonData>,
    q_transforms: Query<&GlobalTransform, (With<PathfindingTarget>, Without<PathfindingSource>)>,
    mut q_enemies: Query<&mut Enemy>,
    mut q_pathfinding_sources: Query<(&Parent, &GlobalTransform, &mut PathfindingSource)>,
) {
    let Ok(target_transform) = q_transforms.get_single() else {
        return;
    };

    for (parent, enemy_transform, mut pf_source) in &mut q_pathfinding_sources {
        let Ok(mut enemy) = q_enemies.get_mut(parent.get()) else {
            continue;
        };

        // let Some(target) = enemy.target else { continue };
        // let Ok(target_transform) = q_transforms.get(target) else {
        //     continue;
        // };

        let path = a_star(
            enemy_transform.translation().truncate(),
            target_transform.translation().truncate(),
            &map_polygon_data.grid_matrix,
            &pf_source.path,
        );
        pf_source.path = Some(path.clone());

        let pos = if path.len() < 2 {
            target_transform.translation().truncate()
        } else {
            // Check triangle
            let path_dir = path[1] - path[0];
            let dir = enemy_transform.translation().truncate() - path[0];

            // Enemy is left to orthogonal vector, meaning it's already passed `path[0]`
            if dir.perp_dot(rotate_vec(path_dir, ROT_MATRIX_LEFT)) >= 0.0
                && dir.perp_dot(rotate_vec(path_dir, ROT_MATRIX_RIGHT)) <= 0.0
            {
                warn!("dir: {}", dir);
                path[1]
            } else if enemy_transform
                .translation()
                .truncate()
                .distance_squared(path[0])
                // TODO: figure out a good value here, also don't use magic numbers, make const or
                // something
                < 1.0
            {
                path[1]
            } else {
                error!(
                    "dir: {}, path_dir: {}, left: {}, right: {}",
                    dir,
                    path_dir,
                    dir.perp_dot(rotate_vec(path_dir, ROT_MATRIX_LEFT)),
                    dir.perp_dot(rotate_vec(path_dir, ROT_MATRIX_RIGHT))
                );
                path[0]
            }
        };

        // info!(
        //     "e_t: {}, p_t: {}, path: {:?}",
        //     enemy_transform.translation().truncate(),
        //     pos,
        //     path
        // );

        enemy.target_pos = pos;
    }
}

fn update_move_directions(
    mut q_enemies: Query<&mut Enemy>,
    q_pathfinding_sources: Query<(&Parent, &GlobalTransform), With<PathfindingSource>>,
) {
    for (parent, transform) in &q_pathfinding_sources {
        let Ok(mut enemy) = q_enemies.get_mut(parent.get()) else {
            continue;
        };
        if enemy.target.is_none() {
            continue;
        }

        enemy.move_direction =
            (enemy.target_pos - transform.translation().truncate()).normalize_or_zero();
    }
}

pub struct EnemyMovementPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_target_positions.run_if(resource_exists::<MapPolygonData>),
                update_move_directions,
                move_enemies,
            )
                .chain()
                .after(EnemyStateSystemSet),
        );
    }
}
