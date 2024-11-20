use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::{
    dude::{Attack, DudeState, ParryState},
    enemy::{Enemy, EnemyCollisionSystemSet},
    world::{
        collisions::{
            Hitbox, HitboxType, Hurtbox, HurtboxType, ENEMY_GROUP, PLAYER_GROUP, WORLD_GROUP,
        },
        PathfindingTarget,
    },
    GameState,
};

use super::{Player, PlayerStateSystemSet};

fn enemy_hitbox_collisions(
    mut q_players: Query<&mut Player>,
    q_enemies: Query<&Enemy>,
    q_hitboxes: Query<&Hitbox>,
    q_hurtboxes: Query<&Hurtbox>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    for ev in ev_collision_events.read() {
        let (source, target, flags) = match ev {
            CollisionEvent::Started(source, target, flags) => (source, target, flags),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        // None of the colliders are sensors, so it can't be hitbox & hurtbox collision.
        if *flags & CollisionEventFlags::SENSOR != CollisionEventFlags::SENSOR {
            continue;
        }

        let player_hurtbox = if let Ok(r) = q_hurtboxes.get(*source) {
            r
        } else if let Ok(r) = q_hurtboxes.get(*target) {
            r
        } else {
            continue;
        };

        let enemy_hitbox = if let Ok(r) = q_hitboxes.get(*source) {
            r
        } else if let Ok(r) = q_hitboxes.get(*target) {
            r
        } else {
            continue;
        };

        let Ok(mut player) = q_players.get_mut(player_hurtbox.root_entity) else {
            continue;
        };
        if player.state_machine.state() == DudeState::Dashing {
            continue;
        }

        let Ok(enemy) = q_enemies.get(enemy_hitbox.root_entity) else {
            continue;
        };

        let HitboxType::Enemy(_attack) = enemy_hitbox.hitbox_type else {
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
            .set_stagger_state(enemy.state_machine.attack_direction());
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
                enemy_hitbox_collisions,
                change_collider_collisions,
            )
                .chain()
                .before(PlayerStateSystemSet)
                .before(EnemyCollisionSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
