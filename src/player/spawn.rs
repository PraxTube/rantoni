use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    world::collisions::{spawn_hitbox_collision, Hitbox, HitboxType, PLAYER_GROUP, WORLD_GROUP},
    GameAssets, GameState,
};

use super::{Player, PlayerAttackState};

#[derive(Component)]
pub struct PlayerHitboxRoot {
    pub root_entity: Entity,
}

fn spawn_player_hitboxes(commands: &mut Commands, player_entity: Entity) -> Entity {
    let hitboxes = [
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(PlayerAttackState::Light1),
                PLAYER_GROUP,
                Vec2::new(10.0, 3.0),
                true,
            ),
            Collider::cuboid(8.0, 2.0),
        ),
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(PlayerAttackState::Light2),
                PLAYER_GROUP,
                Vec2::new(8.0, 1.0),
                true,
            ),
            Collider::cuboid(8.0, 2.0),
        ),
    ];
    commands
        .spawn((
            PlayerHitboxRoot {
                root_entity: player_entity,
            },
            TransformBundle::default(),
        ))
        .push_children(&hitboxes)
        .id()
}

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let player_entity = commands
        .spawn((
            Player::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(0.0),
            SpriteBundle {
                texture: assets.player_texture.clone(),
                ..default()
            },
            TextureAtlas::from(assets.player_layout.clone()),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(6.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(WORLD_GROUP | PLAYER_GROUP, WORLD_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -12.0, 0.0,
            ))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.player_animations[0].clone()).repeat();

    let hitboxes = spawn_player_hitboxes(&mut commands, player_entity);

    commands
        .entity(player_entity)
        .insert(animator)
        .push_children(&[collider, hitboxes]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
