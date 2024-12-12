use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    dude::{DudeState, Health, ParryState},
    enemy::EnemyCollisionSystemSet,
    world::{
        collisions::{HitboxHurtboxEvent, HitboxType, ENEMY_GROUP, PLAYER_GROUP, WORLD_GROUP},
        PathfindingTarget,
    },
    GameState,
};

use super::{Player, PlayerStateSystemSet};

pub const DEFAULT_PLAYER_COLLISION_GROUPS: CollisionGroups =
    CollisionGroups::new(PLAYER_GROUP, WORLD_GROUP.union(ENEMY_GROUP));
const DASHING_PLAYER_COLLISION_GROUPS: CollisionGroups =
    CollisionGroups::new(PLAYER_GROUP, WORLD_GROUP);

fn hitbox_collisions(
    mut q_players: Query<(&mut Player, &mut Health)>,
    mut ev_hitbox_hurtbox: EventReader<HitboxHurtboxEvent>,
) {
    for ev in ev_hitbox_hurtbox.read() {
        let Ok((mut player, mut health)) = q_players.get_mut(ev.hurtbox.root_entity) else {
            continue;
        };
        if player.state_machine.state() == DudeState::Dashing {
            warn!("you got a hurtbox event on player while he is dashing, should never happen, frame delay?");
            continue;
        }

        let HitboxType::Enemy(attack) = ev.hitbox.hitbox_type else {
            error!("hitbox type is not that of enemy, this should never happen");
            continue;
        };

        if player.state_machine.state() == DudeState::Parrying(ParryState::Start) {
            player
                .state_machine
                .set_state(DudeState::Parrying(ParryState::Success));
            continue;
        }

        player
            .state_machine
            .set_stagger_state(ev.hitbox.attack_direction);
        // TODO: Use the actual direction from hitbox source to player.
        player.current_direction = -ev.hitbox.attack_direction;

        health.reduce(attack.to_damage());
    }
}

fn change_collider_collisions(
    q_players: Query<&Player>,
    mut q_colliders: Query<(&mut CollisionGroups, &PathfindingTarget)>,
) {
    for (mut collision_groups, pf_target) in &mut q_colliders {
        let Ok(player) = q_players.get(pf_target.root_entity) else {
            continue;
        };

        let new_collision_groups = match player.state_machine.state() {
            DudeState::Dashing => DASHING_PLAYER_COLLISION_GROUPS,
            _ => DEFAULT_PLAYER_COLLISION_GROUPS,
        };

        *collision_groups = new_collision_groups;
    }
}

pub struct PlayerCollisionsPlugin;

impl Plugin for PlayerCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (hitbox_collisions, change_collider_collisions)
                .chain()
                .before(PlayerStateSystemSet)
                .before(EnemyCollisionSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
