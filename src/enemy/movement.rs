use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{state::EnemyState, Enemy};

fn move_enemies(mut q_enemies: Query<(&mut Velocity, &Enemy)>) {
    for (mut velocity, enemy) in &mut q_enemies {
        if enemy.state == EnemyState::Staggering {
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
