mod attack;
mod state_machine;

pub use state_machine::EnemyStateMachine;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::{Attack, DudeState},
    player::Player,
    world::{PathfindingSource, PathfindingTarget},
};

use super::{Enemy, MAX_AGGRO_DISTANCE, MIN_AGGRO_DISTANCE, MIN_TARGET_DISTANCE};

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
                    transition_run_state,
                    transition_idle_state,
                    reset_new_state,
                )
                    .chain()
                    .in_set(EnemyStateSystemSet),
            )
            .add_systems(Update, (reset_enemey_targets).before(EnemyStateSystemSet));
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

        let attack_direction =
            (enemy.target_pos - transform.translation.truncate()).normalize_or_zero();

        enemy
            .state_machine
            .set_attack(Attack::Heavy3, attack_direction);
    }
}

fn transition_idle_state(mut q_enemies: Query<(&AnimationPlayer2D, &mut Enemy)>) {
    for (animator, mut enemy) in &mut q_enemies {
        if enemy.state_machine.just_changed() {
            continue;
        }

        match enemy.state_machine.state() {
            DudeState::Idling | DudeState::Parrying(_) | DudeState::Dashing => {}
            DudeState::Running => {
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
                    enemy.state_machine.set_state(DudeState::Idling);
                }
            }
            DudeState::Staggering => {
                if enemy.state_machine.stagger_state().is_recovering() {
                    if animator.just_finished() {
                        enemy.state_machine.set_state(DudeState::Idling);
                    }
                } else if enemy.state_machine.stagger_finished() {
                    enemy.state_machine.set_stagger_state_recover();
                }
            }
        }
    }
}

fn transition_run_state(
    q_players: Query<(Entity, &Transform), With<Player>>,
    q_pf_targets: Query<(Entity, &PathfindingTarget)>,
    mut q_enemies: Query<(&Transform, &mut Enemy), Without<Player>>,
    mut q_pf_sources: Query<&mut PathfindingSource>,
) {
    for mut pf_source in &mut q_pf_sources {
        let Ok((enemy_transform, mut enemy)) = q_enemies.get_mut(pf_source.root_entity) else {
            continue;
        };

        if enemy.state_machine.just_changed() {
            continue;
        }
        if enemy.state_machine.state() != DudeState::Idling {
            continue;
        }

        for (pf_target_entity, pf_target) in &q_pf_targets {
            let Ok((player, player_transform)) = q_players.get(pf_target.root_entity) else {
                continue;
            };

            let dis = enemy_transform
                .translation
                .truncate()
                .distance_squared(player_transform.translation.truncate());
            if dis > MIN_AGGRO_DISTANCE.powi(2) && dis < MAX_AGGRO_DISTANCE.powi(2) {
                pf_source.target = Some(pf_target_entity);
                enemy.target = Some(player);
                enemy.state_machine.set_state(DudeState::Running);
            }
        }
    }
}

fn reset_new_state(mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        enemy.state_machine.reset_new_state();
    }
}

// TODO: This is an issue if you want to allow enemy on enemy targets.
// The simplest solution is to just load the transform of the target in another system before this
// one, I think one solution is to have something like a `Target` component? That could then have
// tmp stuff like current transform or some shit like that.
fn reset_enemey_targets(
    q_transforms: Query<&Transform, Without<Enemy>>,
    mut q_enemies: Query<(&Transform, &mut Enemy)>,
    mut q_pf_sources: Query<&mut PathfindingSource>,
) {
    for mut pf_source in &mut q_pf_sources {
        let Ok((transform, mut enemy)) = q_enemies.get_mut(pf_source.root_entity) else {
            continue;
        };

        let Some(target) = enemy.target else {
            continue;
        };
        let Ok(target_transform) = q_transforms.get(target) else {
            continue;
        };

        if transform
            .translation
            .truncate()
            .distance_squared(target_transform.translation.truncate())
            < MIN_TARGET_DISTANCE.powi(2)
        {
            pf_source.target = None;
            enemy.target = None;
        }
    }
}
