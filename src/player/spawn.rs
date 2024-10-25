use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::DudeAnimations,
    world::collisions::{spawn_hurtbox_collision, PLAYER_GROUP, WORLD_GROUP},
    GameAssets, GameState,
};

use super::Player;

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[DudeAnimations::Idle.index()].clone())
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
                ..default()
            },
            TextureAtlas::from(assets.dude_layout.clone()),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(10.0),
            ActiveEvents::COLLISION_EVENTS,
            // TODO: Disable player - enemy collision when the player is sliding
            // Though only start doing this once you have some world collisions (like walls) in the
            // game, otherwise you can't really properly test this.
            CollisionGroups::new(WORLD_GROUP | PLAYER_GROUP, WORLD_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    let hurtbox = spawn_hurtbox_collision(
        &mut commands,
        entity,
        Vec2::new(0.0, 0.0),
        Collider::cuboid(8.0, 24.0),
        PLAYER_GROUP,
    );

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

    commands
        .entity(entity)
        .push_children(&[collider, hurtbox, shadow]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
