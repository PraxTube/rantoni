use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    assets::DudeAnimations,
    state::Attack,
    world::collisions::{
        spawn_hitbox_collision, Hitbox, HitboxDirection, HitboxType, PLAYER_GROUP, WORLD_GROUP,
    },
    GameAssets, GameState,
};

use super::Player;

#[derive(Component)]
pub struct PlayerHitboxRoot;

fn spawn_player_hitboxes(commands: &mut Commands, player_entity: Entity) -> Entity {
    let hitboxes = [
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(Attack::Light1),
                HitboxDirection::Right,
                PLAYER_GROUP,
                Vec2::new(20.0, 14.0),
            ),
            Collider::cuboid(12.0, 3.0),
        ),
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(Attack::Light1),
                HitboxDirection::Left,
                PLAYER_GROUP,
                Vec2::new(-20.0, 14.0),
            ),
            Collider::cuboid(12.0, 3.0),
        ),
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(Attack::Light2),
                HitboxDirection::Right,
                PLAYER_GROUP,
                Vec2::new(8.0, 11.0),
            ),
            Collider::cuboid(9.0, 3.0),
        ),
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(Attack::Light3),
                HitboxDirection::Right,
                PLAYER_GROUP,
                Vec2::new(14.0, 1.0),
            ),
            Collider::cuboid(14.0, 5.0),
        ),
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(Attack::Heavy1),
                HitboxDirection::Right,
                PLAYER_GROUP,
                Vec2::new(12.0, -10.0),
            ),
            Collider::cuboid(6.0, 10.0),
        ),
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(Attack::Heavy2),
                HitboxDirection::Right,
                PLAYER_GROUP,
                Vec2::new(14.0, -8.0),
            ),
            Collider::cuboid(8.0, 4.0),
        ),
        spawn_hitbox_collision(
            commands,
            Hitbox::new(
                player_entity,
                HitboxType::Player(Attack::Heavy3),
                HitboxDirection::Right,
                PLAYER_GROUP,
                Vec2::new(14.0, 8.0),
            ),
            Collider::cuboid(8.0, 8.0),
        ),
    ];
    commands
        .spawn((PlayerHitboxRoot, TransformBundle::default()))
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

    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[DudeAnimations::Idle.index()].clone())
        .repeat();

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
