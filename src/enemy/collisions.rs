use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::{
    player::{Player, PlayerHitboxRoot},
    state::{DudeState, Stagger},
    world::collisions::{Hitbox, HitboxType, Hurtbox},
    GameState,
};

use super::{state::EnemyStateSystemSet, Enemy};

fn player_hitbox_collisions(
    q_players: Query<&Player>,
    mut q_enemies: Query<(&mut Enemy, &mut Stagger), Without<PlayerHitboxRoot>>,
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

        // TODO: Optimize by storing a reference of `collider_entity` in the `Player` struct if you
        // experience bad performance due to collisions. Check out Magus Parvus implementation of
        // collisions. Although then it might be harder to implement multiple players.
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

pub struct EnemyCollisionsPlugin;

impl Plugin for EnemyCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_hitbox_collisions,)
                .before(EnemyStateSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
