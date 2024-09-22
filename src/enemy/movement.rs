use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::state::{DudeState, Stagger};

use super::{state::EnemyStateSystemSet, Enemy, MOVE_SPEED};

fn move_enemies(mut q_enemies: Query<(&mut Velocity, &Enemy, &Stagger)>) {
    for (mut velocity, enemy, stagger) in &mut q_enemies {
        match enemy.state_machine.state() {
            DudeState::Running => {
                velocity.linvel = enemy.move_direction * MOVE_SPEED;
            }
            DudeState::Staggering => {
                velocity.linvel = stagger.direction * stagger.intensity;
            }
            _ => {}
        }
    }
}

fn update_target_positions(q_transforms: Query<&Transform>, mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        let Some(target) = enemy.target else { continue };
        let Ok(target_transform) = q_transforms.get(target) else {
            continue;
        };
        enemy.target_pos = target_transform.translation.truncate();
    }
}

fn update_move_directions(mut q_enemies: Query<(&Transform, &mut Enemy)>) {
    for (transform, mut enemy) in &mut q_enemies {
        if enemy.target.is_none() {
            continue;
        }

        enemy.move_direction =
            (enemy.target_pos - transform.translation.truncate()).normalize_or_zero();
    }
}

pub struct EnemyMovementPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_target_positions,
                update_move_directions,
                move_enemies,
            )
                .chain()
                .after(EnemyStateSystemSet),
        );
    }
}
