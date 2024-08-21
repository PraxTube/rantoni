use bevy::{color::palettes::css::RED, prelude::*};
use bevy_rancic::prelude::YSort;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    world::collision::{HITBOX_GROUP, HURTBOX_GROUP},
    GameAssets, GameState,
};

fn spawn_dummy_enemy(mut commands: Commands, assets: Res<GameAssets>) {
    let hurtbox = commands
        .spawn((
            Collider::cuboid(8.0, 16.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(HURTBOX_GROUP, HITBOX_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -12.0, 0.0,
            ))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.player_animations[0].clone()).repeat();

    commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(0.0),
            animator,
            SpriteBundle {
                texture: assets.player_texture.clone(),
                transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
                sprite: Sprite {
                    color: RED.into(),
                    ..default()
                },
                ..default()
            },
            TextureAtlas::from(assets.player_layout.clone()),
        ))
        .push_children(&[hurtbox]);
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_dummy_enemy,));
    }
}
