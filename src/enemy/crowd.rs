use bevy::prelude::*;

use crate::{
    player::Player,
    world::{PathfindingSource, PathfindingTarget},
};

use super::{state::EnemyStateSystemSet, Enemy, MAX_CHASE_DISTANCE};

#[derive(Debug)]
struct TargetDistance {
    entity: Entity,
    target: Entity,
    distance: f32,
}

#[derive(Resource, Default)]
pub struct EnemyCrowd {
    target_distances: Vec<TargetDistance>,
}

fn reset_enmey_crowd(mut enemy_crowd: ResMut<EnemyCrowd>) {
    enemy_crowd.target_distances.clear();
}

fn update_enemy_crowd_distances(
    mut enemy_crowd: ResMut<EnemyCrowd>,
    q_enemies: Query<(Entity, &Transform, &Enemy)>,
) {
    for (entity, transform, enemy) in &q_enemies {
        if let Some(target) = enemy.target {
            enemy_crowd.target_distances.push(TargetDistance {
                entity,
                target,
                distance: transform
                    .translation
                    .truncate()
                    .distance_squared(enemy.target_pos),
            });
        }
    }
}

fn set_enemy_targets(
    q_players: Query<(Entity, &Transform), With<Player>>,
    mut q_enemies: Query<(&Transform, &mut Enemy), Without<Player>>,
) {
    for (enemy_transform, mut enemy) in &mut q_enemies {
        if enemy.target.is_some() {
            continue;
        }

        if let Some((target, _)) = q_players.iter().find(|(_, t)| {
            let dis = enemy_transform
                .translation
                .truncate()
                .distance_squared(t.translation.truncate());

            dis < MAX_CHASE_DISTANCE.powi(2)
        }) {
            enemy.target = Some(target);
        }
    }
}

fn update_enemy_target_positions(
    q_transforms: Query<&Transform>,
    mut q_enemies: Query<&mut Enemy>,
) {
    for mut enemy in &mut q_enemies {
        if let Some(target) = enemy.target {
            if let Ok(transform) = q_transforms.get(target) {
                enemy.target_pos = transform.translation.truncate();
            }
        }
    }
}

/// Set the target entity to `None` if the target is too far away from the enemy.
fn reset_enemey_targets(mut q_enemies: Query<(&Transform, &mut Enemy)>) {
    for (transform, mut enemy) in &mut q_enemies {
        if enemy.target.is_none() {
            continue;
        };

        if transform
            .translation
            .truncate()
            .distance_squared(enemy.target_pos)
            > MAX_CHASE_DISTANCE.powi(2)
        {
            enemy.target = None;
        }
    }
}

/// Sync the enemy targets with the pathfinding source targets.
/// If enmey target is `None` then this will also set the pf source target to `None`.
/// If it is some then it will set the `PathfindingTarget` entity as the target of the pf source.
///
/// Note: The targets will not match in most cases, as they are not intended to match in the first
/// plae.
fn update_pf_source_targets(
    q_enemies: Query<&Enemy>,
    mut q_pf_sources: Query<&mut PathfindingSource>,
    q_pf_targets: Query<(Entity, &PathfindingTarget)>,
) {
    for mut pf_source in &mut q_pf_sources {
        let Ok(enemy) = q_enemies.get(pf_source.root_entity) else {
            continue;
        };

        let target = enemy.target.and_then(|target| {
            q_pf_targets
                .iter()
                .find(|(_, pf_target)| pf_target.root_entity == target)
                .map(|(entity, _)| entity)
        });

        pf_source.target = target;
    }
}

pub struct EnemyCrowdPlugin;

impl Plugin for EnemyCrowdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyCrowd>()
            .add_systems(
                PreUpdate,
                (reset_enmey_crowd, update_enemy_crowd_distances).chain(),
            )
            .add_systems(
                Update,
                (
                    set_enemy_targets,
                    update_enemy_target_positions,
                    reset_enemey_targets,
                    update_pf_source_targets,
                )
                    .chain()
                    .before(EnemyStateSystemSet),
            );
    }
}
