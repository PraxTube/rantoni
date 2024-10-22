use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::{
    dude::{DudeState, ParryState, Stagger},
    player::Player,
    world::collisions::{Hitbox, HitboxType, Hurtbox},
    GameState,
};

use super::{state::EnemyStateSystemSet, Enemy};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyCollisionSystemSet;

fn player_hitbox_collisions(
    q_players: Query<&Player>,
    mut q_enemies: Query<(&mut Enemy, &mut Stagger)>,
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

        let Ok((mut enemy, mut stagger)) = q_enemies.get_mut(enemy_hurtbox.root_entity) else {
            continue;
        };

        if let HitboxType::Player(attack) = player_hitbox.hitbox_type {
            enemy.state_machine.set_new_state(DudeState::Staggering);
            stagger.update(attack, player.current_direction, 1.0, 1.0);
        }
    }
}

fn enemy_parry_collisions(
    q_players: Query<&Player>,
    mut q_enemies: Query<(&mut Enemy, &mut Stagger)>,
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

        let Ok((mut enemy, mut enemy_stagger)) = q_enemies.get_mut(enemy_hitbox.root_entity) else {
            continue;
        };

        let HitboxType::Enemy(_attack) = enemy_hitbox.hitbox_type else {
            continue;
        };

        if player.state_machine.state() == DudeState::Parrying
            && player.state_machine.parry_state() == ParryState::Success
        {
            enemy.state_machine.set_new_state(DudeState::Staggering);
            enemy_stagger.set_stance_break();
        }
    }
}

pub struct EnemyCollisionsPlugin;

impl Plugin for EnemyCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_hitbox_collisions, enemy_parry_collisions)
                .before(EnemyStateSystemSet)
                .in_set(EnemyCollisionSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
