use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::{
    dude::{DudeState, ParryState, StaggerState},
    player::Player,
    world::collisions::{Hitbox, HitboxType, Hurtbox, HurtboxType},
    GameState,
};

use super::{state::EnemyStateSystemSet, Enemy};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyCollisionSystemSet;

fn player_hitbox_collisions(
    q_players: Query<&Player>,
    mut q_enemies: Query<&mut Enemy>,
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

        let player_hitbox = if let Ok(r) = q_hitboxes.get(*source) {
            r
        } else if let Ok(r) = q_hitboxes.get(*target) {
            r
        } else {
            continue;
        };

        let enemy_hurtbox = if let Ok(r) = q_hurtboxes.get(*source) {
            r
        } else if let Ok(r) = q_hurtboxes.get(*target) {
            r
        } else {
            continue;
        };

        let Ok(player) = q_players.get(player_hitbox.root_entity) else {
            continue;
        };

        let Ok(mut enemy) = q_enemies.get_mut(enemy_hurtbox.root_entity) else {
            continue;
        };

        if let HitboxType::Player(attack) = player_hitbox.hitbox_type {
            enemy
                .state_machine
                .set_stagger_state(attack, player.current_direction, 1.0, 1.0);
        }
    }
}

fn enemy_parry_collisions(
    q_players: Query<&Player>,
    mut q_enemies: Query<&mut Enemy>,
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

        let Ok(player) = q_players.get(player_hurtbox.root_entity) else {
            continue;
        };

        let Ok(mut enemy) = q_enemies.get_mut(enemy_hitbox.root_entity) else {
            continue;
        };

        let HitboxType::Enemy(_attack) = enemy_hitbox.hitbox_type else {
            continue;
        };

        if player.state_machine.state() == DudeState::Parrying(ParryState::Success) {
            enemy.state_machine.set_stagger_stance_break_state();
        }
    }
}

fn change_hurtbox_collisions(
    q_enemies: Query<&Enemy>,
    mut q_hurtboxes: Query<(&mut CollisionGroups, &mut ColliderDebugColor, &Hurtbox)>,
) {
    for (mut collision_groups, mut collider_color, hurtbox) in &mut q_hurtboxes {
        let Ok(enemy) = q_enemies.get(hurtbox.root_entity) else {
            continue;
        };

        let hurtbox_type = match enemy.state_machine.state() {
            DudeState::Staggering => match enemy.state_machine.stagger_state() {
                StaggerState::Fall => HurtboxType::Fallen,
                StaggerState::FallRecover => HurtboxType::Fallen,
                _ => HurtboxType::Normal,
            },
            _ => HurtboxType::Normal,
        };

        // TODO: Is this expensive? Are we allocating new stuff whenever we do this or is it more
        // similar to when you set an integer? If it's the former, then you should probably check
        // if the variable is already the corresponding thing and not do anything if it already
        // matches.
        if hurtbox.hurtbox_type != hurtbox_type {
            *collision_groups = COLLISION_GROUPS_NONE;
            *collider_color = COLLIDER_COLOR_BLACK;
        } else {
            *collision_groups = hurtbox.collision_groups;
            *collider_color = COLLIDER_COLOR_WHITE;
        }
    }
}

pub struct EnemyCollisionsPlugin;

impl Plugin for EnemyCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                change_hurtbox_collisions,
                player_hitbox_collisions,
                enemy_parry_collisions,
            )
                .chain()
                .before(EnemyStateSystemSet)
                .in_set(EnemyCollisionSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
