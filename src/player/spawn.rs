use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::{Health, PlayerAnimations},
    world::{
        collisions::{spawn_hurtbox_collision, Hurtbox},
        PathfindingTarget,
    },
    GameAssets, GameState,
};

use super::{collisions::DEFAULT_PLAYER_COLLISION_GROUPS, Player, HEALTH};

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[PlayerAnimations::Idle.index()].clone())
        .repeat();

    let entity = commands
        .spawn((
            Player::default(),
            Health::new(HEALTH),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Dominance::group(1),
            animator,
            YSort(0.0),
            SpriteBundle {
                texture: assets.dude_textures[0].clone(),
                transform: Transform::from_xyz(565.4461, 264.66992, 0.0),
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
            DEFAULT_PLAYER_COLLISION_GROUPS,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    let hurtbox = spawn_hurtbox_collision(
        &mut commands,
        Hurtbox::new(entity),
        Vec2::new(0.0, 0.0),
        Collider::cuboid(10.0, 30.0),
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
        .push_children(&[collider, hurtbox, shadow]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn_player);
    }
}
