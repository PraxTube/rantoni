use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{world::collision::WORLD_GROUP, GameAssets, GameState};

use super::Player;

fn spawn_player(world: &mut World) {
    let collider = world
        .spawn((
            Collider::ball(6.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(WORLD_GROUP, WORLD_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -12.0, 0.0,
            ))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator
        .play(world.resource::<GameAssets>().player_animations[0].clone())
        .repeat();

    let player = Player::from_world(world);

    world
        .spawn((
            player,
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(0.0),
            animator,
            SpriteBundle {
                texture: world.resource::<GameAssets>().player_texture.clone(),
                ..default()
            },
            TextureAtlas::from(world.resource::<GameAssets>().player_layout.clone()),
        ))
        .push_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
