use bevy::prelude::*;

use crate::{
    dude::{DudeState, Health, ParryState},
    player::Player,
    world::collisions::{HitboxHurtboxEvent, HitboxType},
    GameState,
};

use super::{state::EnemyStateSystemSet, Enemy};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyCollisionSystemSet;

fn hitbox_collisions(
    mut q_enemies: Query<(&mut Enemy, &mut Health)>,
    mut ev_hitbox_hurtbox: EventReader<HitboxHurtboxEvent>,
) {
    for ev in ev_hitbox_hurtbox.read() {
        let Ok((mut enemy, mut health)) = q_enemies.get_mut(ev.hurtbox.root_entity) else {
            continue;
        };

        if let HitboxType::Player(attack) = ev.hitbox.hitbox_type {
            enemy
                .state_machine
                .set_stagger_state(attack, ev.hitbox.attack_direction, 1.0, 1.0);
            health.reduce(attack.to_damage());
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
            error!("hitbox type is not that of enemy, this should never happen");
            continue;
        };

        if player.state_machine.state() == DudeState::Parrying(ParryState::Success) {
            enemy.state_machine.set_stagger_stance_break_state();
        }
    }
}

pub struct EnemyCollisionsPlugin;

impl Plugin for EnemyCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (hitbox_collisions, enemy_parry_collisions)
                .chain()
                .before(EnemyStateSystemSet)
                .in_set(EnemyCollisionSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
