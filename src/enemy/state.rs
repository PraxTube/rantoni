use bevy::prelude::*;

use super::Enemy;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum EnemyState {
    #[default]
    Idling,
    Staggering,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyStateSystemSet;

fn transition_idle_state(time: Res<Time>, mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        if enemy.state == EnemyState::Staggering {
            enemy.stagger.timer.tick(time.delta());
            if enemy.stagger.timer.just_finished() {
                enemy.state = EnemyState::Idling;
            }
        }
    }
}

pub struct EnemyStatePlugin;

impl Plugin for EnemyStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (transition_idle_state,).in_set(EnemyStateSystemSet));
    }
}
