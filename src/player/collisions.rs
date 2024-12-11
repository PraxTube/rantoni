use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    dude::{Attack, DudeState, ParryState},
    enemy::EnemyCollisionSystemSet,
    world::{
        collisions::{
            HitboxHurtboxEvent, HitboxType, Hurtbox, HurtboxType, ENEMY_GROUP, PLAYER_GROUP,
            WORLD_GROUP,
        },
        PathfindingTarget,
    },
    GameState,
};

use super::{Player, PlayerStateSystemSet};

fn hitbox_collisions(
    mut q_players: Query<&mut Player>,
    mut ev_hitbox_hurtbox: EventReader<HitboxHurtboxEvent>,
) {
    for ev in ev_hitbox_hurtbox.read() {
        let Ok(mut player) = q_players.get_mut(ev.hurtbox.root_entity) else {
            continue;
        };
        if player.state_machine.state() == DudeState::Dashing {
            warn!("you got a hurtbox event on player while he is dashing, should never happen, frame delay?");
            continue;
        }

        let HitboxType::Enemy(_attack) = ev.hitbox.hitbox_type else {
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
    }
}

fn change_hurtbox_collisions(
    q_players: Query<&Player>,
    mut q_hurtboxes: Query<(&mut CollisionGroups, &mut ColliderDebugColor, &Hurtbox)>,
) {
    for (mut collision_groups, mut collider_color, hurtbox) in &mut q_hurtboxes {
        let Ok(player) = q_players.get(hurtbox.root_entity) else {
            continue;
        };

        let hurtbox_type = match player.state_machine.state() {
            DudeState::Idling => HurtboxType::Normal,
            DudeState::Stalking => HurtboxType::Normal,
            DudeState::Running => HurtboxType::Normal,
            DudeState::Attacking => match player.state_machine.attack() {
                Attack::Dropkick => HurtboxType::Jumping,
                Attack::Hammerfist => HurtboxType::Jumping,
                _ => HurtboxType::Normal,
            },
            DudeState::Recovering => HurtboxType::Normal,
            DudeState::Staggering => HurtboxType::Normal,
            DudeState::Dashing => HurtboxType::None,
            DudeState::Parrying(_) => HurtboxType::Normal,
            DudeState::Dying => HurtboxType::None,
        };

        if hurtbox.hurtbox_type != hurtbox_type {
            *collision_groups = COLLISION_GROUPS_NONE;
            *collider_color = COLLIDER_COLOR_BLACK;
        } else {
            *collision_groups = hurtbox.collision_groups;
            *collider_color = COLLIDER_COLOR_WHITE;
        }
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

        let (memberships, filters) = match player.state_machine.state() {
            DudeState::Dashing => (WORLD_GROUP | PLAYER_GROUP, WORLD_GROUP),
            _ => (WORLD_GROUP | PLAYER_GROUP, WORLD_GROUP | ENEMY_GROUP),
        };

        collision_groups.memberships = memberships;
        collision_groups.filters = filters;
    }
}

pub struct PlayerCollisionsPlugin;

impl Plugin for PlayerCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                change_hurtbox_collisions,
                hitbox_collisions,
                change_collider_collisions,
            )
                .chain()
                .before(PlayerStateSystemSet)
                .before(EnemyCollisionSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
