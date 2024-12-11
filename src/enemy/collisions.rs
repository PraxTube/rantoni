use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    dude::{DudeState, ParryState, StaggerState},
    player::Player,
    world::collisions::{
        HitboxHurtboxEvent, HitboxType, Hurtbox, HurtboxType, HURTBOX_COLLISION_GROUPS,
    },
    GameState,
};

use super::{state::EnemyStateSystemSet, Enemy};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyCollisionSystemSet;

fn hitbox_collisions(
    mut q_enemies: Query<&mut Enemy>,
    mut ev_hitbox_hurtbox: EventReader<HitboxHurtboxEvent>,
) {
    for ev in ev_hitbox_hurtbox.read() {
        let Ok(mut enemy) = q_enemies.get_mut(ev.hurtbox.root_entity) else {
            continue;
        };

        // TODO: Do you want to send an event that somebody got hit?
        // Perf should be fine, would also allow health stuff, would be cleaner I think?

        if let HitboxType::Player(attack) = ev.hitbox.hitbox_type {
            enemy
                .state_machine
                .set_stagger_state(attack, ev.hitbox.attack_direction, 1.0, 1.0);
        }
    }
}

fn enemy_parry_collisions(
    q_players: Query<&Player>,
    mut q_enemies: Query<&mut Enemy>,
    mut ev_hitbox_hurtbox: EventReader<HitboxHurtboxEvent>,
) {
    for ev in ev_hitbox_hurtbox.read() {
        let Ok(player) = q_players.get(ev.hurtbox.root_entity) else {
            continue;
        };

        let Ok(mut enemy) = q_enemies.get_mut(ev.hitbox.root_entity) else {
            continue;
        };

        let HitboxType::Enemy(_attack) = ev.hitbox.hitbox_type else {
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

        if hurtbox.hurtbox_type != hurtbox_type {
            *collision_groups = COLLISION_GROUPS_NONE;
            *collider_color = COLLIDER_COLOR_BLACK;
        } else {
            *collision_groups = HURTBOX_COLLISION_GROUPS;
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
                hitbox_collisions,
                enemy_parry_collisions,
            )
                .chain()
                .before(EnemyStateSystemSet)
                .in_set(EnemyCollisionSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
