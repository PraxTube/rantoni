use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const HURTBOX_GROUP: Group = Group::GROUP_1;
pub const HITBOX_GROUP: Group = Group::GROUP_2;
pub const WORLD_GROUP: Group = Group::GROUP_3;

pub struct WorldCollisionPlugin;

impl Plugin for WorldCollisionPlugin {
    fn build(&self, _app: &mut App) {}
}
