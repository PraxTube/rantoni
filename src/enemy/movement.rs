use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::state::DudeState;

use super::Enemy;

fn move_enemies(mut q_enemies: Query<(&mut Velocity, &Enemy)>) {
    for (mut velocity, enemy) in &mut q_enemies {
        if enemy.state_machine.state() == DudeState::Staggering {
            velocity.linvel = enemy.stagger.direction * enemy.stagger.intensity;
        }
    }
}

pub struct EnemyMovementPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_enemies,));
    }
}
