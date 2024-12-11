use bevy::prelude::*;

use crate::GameAssets;

fn spawn_enemy_death_effect(mut commands: Commands, assets: Res<GameAssets>) {
    //
}

// fn despawn_enemies(mut commands: Commands)

pub struct EnemyHealthPlugin;

impl Plugin for EnemyHealthPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}
