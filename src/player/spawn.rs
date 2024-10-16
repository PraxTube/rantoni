use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    assets::DudeAnimations,
    world::collisions::{PLAYER_GROUP, WORLD_GROUP},
    GameAssets, GameState,
};

use super::Player;

#[derive(Component)]
pub struct PlayerHitboxRoot;

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[DudeAnimations::Idle.index()].clone())
        .repeat();

    let player_entity = commands
        .spawn((
            Player::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            animator,
            YSort(0.0),
            SpriteBundle {
                texture: assets.dude_textures[0].clone(),
                ..default()
            },
            TextureAtlas::from(assets.dude_layout.clone()),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(10.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(WORLD_GROUP | PLAYER_GROUP, WORLD_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    commands.entity(player_entity).push_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
