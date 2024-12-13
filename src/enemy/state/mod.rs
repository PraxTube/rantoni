mod attack;
mod state_machine;

pub use state_machine::EnemyStateMachine;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::{Attack, DudeState},
    player::Player,
};

use super::{Enemy, ATTACK_DISTANCE, MIN_CHASE_DISTANCE};

pub struct EnemyStatePlugin;

impl Plugin for EnemyStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(attack::EnemyAttackStatePlugin)
            .add_systems(PreUpdate, reset_just_changed)
            .add_systems(
                Update,
                (
                    transition_stagger_state,
                    transition_attack_state,
                    transition_stalking_state,
                    transition_run_state,
                    transition_idle_state,
                    transition_death_state,
                    reset_new_state,
                )
                    .chain()
                    .in_set(EnemyStateSystemSet),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyStateSystemSet;

fn reset_just_changed(mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        enemy.state_machine.set_just_changed(false);
    }
}

fn transition_stagger_state(mut q_enemies: Query<(&mut AnimationPlayer2D, &mut Enemy)>) {
    for (mut animator, mut enemy) in &mut q_enemies {
        if enemy.state_machine.just_changed() {
            continue;
        }
        let Some(new_state) = enemy.state_machine.new_state() else {
            continue;
        };
        if new_state != DudeState::Staggering {
            continue;
        }

        enemy.state_machine.reset_attack_timer();

        if enemy.state_machine.state() == DudeState::Staggering {
            animator.replay();
        } else {
            enemy.state_machine.set_state(DudeState::Staggering);
        }
    }
}

fn transition_attack_state(mut q_enemies: Query<(&Transform, &mut Enemy)>) {
    for (transform, mut enemy) in &mut q_enemies {
        if !enemy.state_machine.can_attack() {
            continue;
        }
        if !enemy.state_machine.attack_timer_finished() {
            continue;
        }

        if (enemy.target_pos - transform.translation.truncate()).length_squared()
            > ATTACK_DISTANCE.powi(2)
        {
            continue;
        }

        let attack_direction =
            (enemy.target_pos - transform.translation.truncate()).normalize_or_zero();

        enemy
            .state_machine
            .set_attack(Attack::Light1, attack_direction);
    }
}

fn transition_stalking_state(mut q_enemies: Query<(&Transform, &mut Enemy)>) {
    for (transform, mut enemy) in &mut q_enemies {
        if enemy.state_machine.just_changed() {
            continue;
        }
        if enemy.target.is_none() {
            continue;
        }
        if enemy.state_machine.state() != DudeState::Idling {
            continue;
        }

        if transform
            .translation
            .truncate()
            .distance_squared(enemy.target_pos)
            < MIN_CHASE_DISTANCE.powi(2)
        {
            enemy.state_machine.set_state(DudeState::Stalking);
        }
    }
}

fn transition_run_state(mut q_enemies: Query<(&Transform, &mut Enemy), Without<Player>>) {
    for (transform, mut enemy) in &mut q_enemies {
        if enemy.state_machine.just_changed() {
            continue;
        }
        if enemy.target.is_none() {
            continue;
        }
        if enemy.state_machine.state() != DudeState::Idling
            && enemy.state_machine.state() != DudeState::Stalking
        {
            continue;
        }

        let dis = transform
            .translation
            .truncate()
            .distance_squared(enemy.target_pos);
        if dis > MIN_CHASE_DISTANCE.powi(2) {
            enemy.state_machine.set_state(DudeState::Running);
        }
    }
}

fn transition_death_state() {}

fn transition_idle_state(mut q_enemies: Query<(&AnimationPlayer2D, &mut Enemy)>) {
    for (animator, mut enemy) in &mut q_enemies {
        if enemy.state_machine.just_changed() {
            continue;
        }

        match enemy.state_machine.state() {
            DudeState::Idling | DudeState::Parrying(_) | DudeState::Dashing | DudeState::Dying => {}
            DudeState::Running => {
                if enemy.target.is_none() {
                    enemy.state_machine.set_state(DudeState::Idling);
                }
            }
            DudeState::Stalking => {
                if enemy.target.is_none() {
                    enemy.state_machine.set_state(DudeState::Idling);
                }
            }
            DudeState::Attacking => {
                if animator.just_finished() {
                    enemy.state_machine.set_state(DudeState::Recovering);
                }
            }
            DudeState::Recovering => {
                if animator.just_finished() {
                    enemy.state_machine.reset_attack_timer();
                    enemy.state_machine.set_state(DudeState::Stalking);
                }
            }
            DudeState::Staggering => {
                if enemy.state_machine.stagger_state().is_recovering() {
                    if animator.just_finished() {
                        enemy.state_machine.set_state(DudeState::Stalking);
                    }
                } else if enemy.state_machine.stagger_finished() {
                    enemy.state_machine.set_stagger_state_recover();
                }
            }
        }
    }
}

fn reset_new_state(mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        enemy.state_machine.reset_new_state();
    }
}
