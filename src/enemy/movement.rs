use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    dude::DudeState,
    world::{a_star, MapPolygonData, PathfindingSource, PathfindingTarget},
};

use super::{state::EnemyStateSystemSet, Enemy, MOVE_SPEED};

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

fn update_target_positions(
    map_polygon_data: Res<MapPolygonData>,
    q_transforms: Query<&GlobalTransform, (With<PathfindingTarget>, Without<PathfindingSource>)>,
    mut q_enemies: Query<&mut Enemy>,
    q_pathfinding_sources: Query<(&Parent, &GlobalTransform), With<PathfindingSource>>,
) {
    let Ok(target_transform) = q_transforms.get_single() else {
        error!("not good");
        return;
    };

    for (parent, enemy_transform) in &q_pathfinding_sources {
        let Ok(mut enemy) = q_enemies.get_mut(parent.get()) else {
            warn!("no enemy");
            continue;
        };

        // let Some(target) = enemy.target else { continue };
        // let Ok(target_transform) = q_transforms.get(target) else {
        //     continue;
        // };

        let path = a_star(
            enemy_transform.translation().truncate(),
            target_transform.translation().truncate(),
            &map_polygon_data.navmesh_polygons,
            &map_polygon_data.adjacency_graph,
        );

        let pos = if path.is_empty() {
            target_transform.translation().truncate()
        } else if enemy_transform
            .translation()
            .truncate()
            .distance_squared(path[0].1)
            < 1e-05
        {
            if path.len() == 1 {
                target_transform.translation().truncate()
            } else {
                path[1].1
            }
        } else {
            path[0].1
        };

        // info!("p: {}, e: {}", pos, enemy_transform.translation.truncate());
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
        info!("move_dir: {}", enemy.move_direction);
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
