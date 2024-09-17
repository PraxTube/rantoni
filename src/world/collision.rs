use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const HURTBOX_GROUP: Group = Group::GROUP_1;
pub const HITBOX_GROUP: Group = Group::GROUP_2;
pub const WORLD_GROUP: Group = Group::GROUP_3;
pub const PLAYER_GROUP: Group = Group::GROUP_4;
pub const ENEMY_GROUP: Group = Group::GROUP_5;

pub fn spawn_hitbox_collision(
    commands: &mut Commands,
    offset: Vec2,
    collider: Collider,
    group: Group,
) -> Entity {
    commands
        .spawn((
            collider,
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(HITBOX_GROUP | group, HURTBOX_GROUP),
            TransformBundle::from_transform(Transform::from_translation(offset.extend(0.0))),
        ))
        .id()
}

pub struct WorldCollisionPlugin;

impl Plugin for WorldCollisionPlugin {
    fn build(&self, _app: &mut App) {}
}
