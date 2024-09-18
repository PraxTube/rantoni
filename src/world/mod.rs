pub mod collisions;
pub mod stagger;

use bevy::prelude::*;
use bevy_rancic::prelude::*;

use crate::{player::Player, GameAssets, GameState};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(collisions::WorldCollisionPlugin)
            .add_systems(OnExit(GameState::AssetLoading), (spawn_dummy_backgrounds,))
            .add_systems(
                PostUpdate,
                update_camera_target.in_set(CameraSystem::TargetUpdate),
            );
    }
}

fn spawn_dummy_background(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    commands.spawn(SpriteBundle {
        texture: assets.dummy_background.clone(),
        transform: Transform::from_translation(pos),
        ..default()
    });
}

fn spawn_dummy_backgrounds(mut commands: Commands, assets: Res<GameAssets>) {
    let chunks = 32;
    for i in -chunks..=chunks {
        for j in -chunks..=chunks {
            let pos = Vec2::new(i as f32 * 32.0, j as f32 * 32.0);
            spawn_dummy_background(&mut commands, &assets, pos.extend(-10.0));
        }
    }
}

fn update_camera_target(
    mut camera_shake: ResMut<CameraShake>,
    q_player: Query<&Transform, With<Player>>,
) {
    let transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    camera_shake.update_target(transform.translation.truncate());
}
