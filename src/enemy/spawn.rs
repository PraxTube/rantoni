use bevy::{color::palettes::css::RED, prelude::*};
use bevy_rancic::prelude::YSort;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    assets::DudeAnimations,
    state::Stagger,
    world::collisions::{spawn_hurtbox_collision, ENEMY_GROUP, WORLD_GROUP},
    GameAssets, GameState,
};

use super::Enemy;

fn spawn_dummy_enemy(mut commands: Commands, assets: Res<GameAssets>) {
    let entity = commands
        .spawn((
            Enemy::default(),
            Stagger::default(),
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
                transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
                sprite: Sprite {
                    color: RED.into(),
                    ..default()
                },
                ..default()
            },
            TextureAtlas::from(assets.dude_layout.clone()),
        ))
        .id();

    let hurtbox = spawn_hurtbox_collision(
        &mut commands,
        entity,
        Vec2::new(0.0, 0.0),
        Collider::cuboid(8.0, 16.0),
        ENEMY_GROUP,
    );

    let collider = commands
        .spawn((
            Collider::ball(6.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(WORLD_GROUP | ENEMY_GROUP, WORLD_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -12.0, 0.0,
            ))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[DudeAnimations::Idle.index()].clone())
        .repeat();

    commands
        .entity(entity)
        .insert(animator)
        .push_children(&[collider, hurtbox]);
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_dummy_enemy,));
    }
}
