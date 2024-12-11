use bevy::prelude::*;

use crate::dude::Health;

use super::Enemy;

fn despawn_enemies(mut commands: Commands, q_enemies: Query<(Entity, &Health), With<Enemy>>) {
    for (entity, health) in &q_enemies {
        if health.health == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct EnemyHealthPlugin;

impl Plugin for EnemyHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (despawn_enemies,));
    }
}
