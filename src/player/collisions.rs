use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::{
    dude::{Attack, DudeState, JumpingState, ParryState, StaggerState},
    enemy::{Enemy, EnemyCollisionSystemSet},
    world::collisions::{Hitbox, HitboxType, Hurtbox, HurtboxType},
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
                Attack::Slide => HurtboxType::Fallen,
                Attack::Dropkick => HurtboxType::Jumping,
                Attack::Kneekick => HurtboxType::Jumping,
                _ => HurtboxType::Normal,
            },
            DudeState::Recovering => HurtboxType::Normal,
            DudeState::Staggering => match player.state_machine.stagger_state() {
                StaggerState::Fall => HurtboxType::Fallen,
                StaggerState::FallRecover => HurtboxType::Fallen,
                _ => HurtboxType::Normal,
            },
            DudeState::Parrying(_) => HurtboxType::Normal,
            DudeState::Jumping(jumping_state) => match jumping_state {
                JumpingState::Start => HurtboxType::Jumping,
                _ => HurtboxType::Normal,
            },
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

pub struct PlayerCollisionsPlugin;

impl Plugin for PlayerCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (change_hurtbox_collisions, enemy_hitbox_collisions)
                .chain()
                .before(PlayerStateSystemSet)
                .before(EnemyCollisionSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
