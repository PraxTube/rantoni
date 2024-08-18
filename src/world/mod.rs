use bevy::prelude::*;
use bevy_rancic::prelude::*;

use crate::{player::Player, GameAssets, GameState};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), (spawn_dummy_background,))
            .add_systems(
                PostUpdate,
                update_camera_target.in_set(CameraSystem::TargetUpdate),
            );
    }
}

fn spawn_dummy_background(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(SpriteBundle {
        texture: assets.dummy_background.clone(),
        transform: Transform::from_translation(Vec3::new(150.0, 0.0, 0.0)),
        ..default()
    });
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
