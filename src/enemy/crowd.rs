use bevy::prelude::*;

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

fn debug(enemy_crowd: Res<EnemyCrowd>) {
    info!("{:?}", enemy_crowd.target_distances);
}

pub struct EnemyCrowdPlugin;

impl Plugin for EnemyCrowdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyCrowd>()
            .add_systems(
                PreUpdate,
                (reset_enmey_crowd, update_enemy_crowd_distances, debug).chain(),
            )
            .add_systems(
                Update,
                (update_enemy_target_positions, reset_enemey_targets)
                    .chain()
                    .before(EnemyStateSystemSet),
            );
    }
}
