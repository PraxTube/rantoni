use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::PlayerAnimations,
    world::{
        collisions::{
            spawn_hurtbox_collision, Hurtbox, HurtboxType, ENEMY_GROUP, PLAYER_GROUP, WORLD_GROUP,
        },
        PathfindingTarget,
    },
    GameAssets, GameState,
};

use super::Player;

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[PlayerAnimations::Idle.index()].clone())
        .repeat();

    let entity = commands
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
                transform: Transform::from_xyz(100.0, 100.0, 0.0),
                ..default()
            },
            TextureAtlas::from(assets.dude_layout.clone()),
        ))
        .id();

    let collider = commands
        .spawn((
            PathfindingTarget {
                root_entity: entity,
            },
            Collider::ball(16.0),
            CollisionGroups::new(WORLD_GROUP | PLAYER_GROUP, WORLD_GROUP | ENEMY_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    let hurtbox_default = spawn_hurtbox_collision(
        &mut commands,
        Hurtbox::new(entity, HurtboxType::Normal),
        Vec2::new(0.0, 0.0),
        Collider::cuboid(10.0, 30.0),
    );
    let hurtbox_jumping = spawn_hurtbox_collision(
        &mut commands,
        Hurtbox::new(entity, HurtboxType::Jumping),
        Vec2::new(0.0, 22.0),
        Collider::cuboid(12.0, 14.0),
    );

    let shadow = commands
        .spawn((
            YSortChild(-100.0),
            SpriteBundle {
                texture: assets.dude_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -27.0, 0.0)),
                ..default()
            },
        ))
        .id();

    commands
        .entity(entity)
        .push_children(&[collider, hurtbox_default, hurtbox_jumping, shadow]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn_player);
    }
}
