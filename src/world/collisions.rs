use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::dude::Attack;

pub const HURTBOX_GROUP: Group = Group::GROUP_1;
pub const HITBOX_GROUP: Group = Group::GROUP_2;
pub const WORLD_GROUP: Group = Group::GROUP_3;
pub const PLAYER_GROUP: Group = Group::GROUP_4;
pub const ENEMY_GROUP: Group = Group::GROUP_5;

#[derive(PartialEq, Eq, Clone)]
pub enum HitboxType {
    Player(Attack),
    // Enemy(EnemyHitbox),
    #[allow(dead_code)]
    Placeholder,
}

#[derive(Component, Clone)]
pub struct Hitbox {
    pub root_entity: Entity,
    pub hitbox_type: HitboxType,
    pub memberships: Group,
    pub offset: Vec2,
    filters: Group,
}

#[derive(Component)]
pub struct Hurtbox {
    pub root_entity: Entity,
}

impl Hitbox {
    pub fn new(root_entity: Entity, hitbox_type: HitboxType, group: Group, offset: Vec2) -> Self {
        Self {
            root_entity,
            hitbox_type,
            memberships: group,
            offset,
            filters: HURTBOX_GROUP,
        }
    }
}

pub fn spawn_hitbox_collision(
    commands: &mut Commands,
    hitbox: Hitbox,
    collider: Collider,
) -> Entity {
    let mut hitbox = hitbox.clone();
    hitbox.memberships |= HITBOX_GROUP;
    let transform = Transform::from_translation(hitbox.offset.extend(0.0));
    commands
        .spawn((
            hitbox.clone(),
            collider,
            Sensor,
            // TODO: Figure out if we need this, it might be that this leads to two events of the
            // exact same info being triggered, so it might be necessary to only have this on the
            // HURTBOX for example and no events on the HITBOX
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(hitbox.memberships, hitbox.filters),
            COLLIDER_COLOR_WHITE,
            TransformBundle::from_transform(transform),
        ))
        .id()
}

pub fn spawn_hurtbox_collision(
    commands: &mut Commands,
    root_entity: Entity,
    offset: Vec2,
    collider: Collider,
    group: Group,
) -> Entity {
    commands
        .spawn((
            Hurtbox { root_entity },
            collider,
            Sensor,
            // TODO: Figure out if we need this, it might be that this leads to two events of the
            // exact same info being triggered, so it might be necessary to only have this on the
            // HURTBOX for example and no events on the HITBOX
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(HURTBOX_GROUP | group, HITBOX_GROUP),
            TransformBundle::from_transform(Transform::from_translation(offset.extend(0.0))),
        ))
        .id()
}

pub struct WorldCollisionPlugin;

impl Plugin for WorldCollisionPlugin {
    fn build(&self, _app: &mut App) {}
}
