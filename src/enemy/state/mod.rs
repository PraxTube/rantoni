mod state_machine;

pub use state_machine::EnemyStateMachine;

use bevy::prelude::*;

use crate::state::DudeState;

use super::Enemy;

pub struct EnemyStatePlugin;

impl Plugin for EnemyStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                reset_just_changed,
                transition_idle_state,
                transition_run_state,
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

fn transition_idle_state(time: Res<Time>, mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        if enemy.state_machine.state() == DudeState::Staggering {
            enemy.stagger.timer.tick(time.delta());
            if enemy.stagger.timer.just_finished() {
                enemy.state_machine.set_state(DudeState::Idling);
            }
        }
    }
}

fn transition_run_state() {}
