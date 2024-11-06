use bevy::{color::palettes::css::RED, prelude::*};
use bevy_rancic::prelude::{YSort, YSortChild};
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::DudeAnimations,
    world::{
        collisions::{spawn_hurtbox_collision, Hurtbox, HurtboxType, ENEMY_GROUP, WORLD_GROUP},
        PathfindingSource,
    },
    GameAssets, GameState,
};

use super::Enemy;

fn spawn_dummy_enemy(mut commands: Commands, assets: Res<GameAssets>) {
    let entity = commands
        .spawn((
            Enemy::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Damping {
                linear_damping: 100.0,
                ..default()
            },
            YSort(0.0),
            SpriteBundle {
                texture: assets.dude_textures[0].clone(),
                sprite: Sprite {
                    color: RED.into(),
                    ..default()
                },
                ..default()
            },
            TextureAtlas::from(assets.dude_layout.clone()),
        ))
        .id();

    let hurtbox_normal = spawn_hurtbox_collision(
        &mut commands,
        Hurtbox::new(entity, HurtboxType::Normal, ENEMY_GROUP),
        Vec2::new(0.0, 0.0),
        Collider::cuboid(8.0, 24.0),
    );
    let hurtbox_fallen = spawn_hurtbox_collision(
        &mut commands,
        Hurtbox::new(entity, HurtboxType::Fallen, ENEMY_GROUP),
        Vec2::new(0.0, -16.0),
        Collider::cuboid(20.0, 14.0),
    );

    let collider = commands
        .spawn((
            PathfindingSource::default(),
            Collider::ball(10.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(WORLD_GROUP | ENEMY_GROUP, WORLD_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    let shadow = commands
        .spawn((
            YSortChild(-100.0),
            SpriteBundle {
                texture: assets.dude_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -18.0, 0.0)),
                ..default()
            },
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[DudeAnimations::Idle.index()].clone())
        .repeat();

    commands.entity(entity).insert(animator).push_children(&[
        collider,
        hurtbox_normal,
        hurtbox_fallen,
        shadow,
    ]);
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_dummy_enemy,));
    }
}
