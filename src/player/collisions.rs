use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::{
    dude::{DudeState, ParryState, Stagger},
    enemy::Enemy,
    world::collisions::{Hitbox, HitboxType, Hurtbox},
    GameState,
};

use super::{Player, PlayerStateSystemSet};

fn enemy_hitbox_collisions(
    mut q_players: Query<(&mut Player, &mut Stagger)>,
    mut q_enemies: Query<(&mut Enemy, &mut Stagger), Without<Player>>,
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

        let Ok((mut player, mut player_stagger)) = q_players.get_mut(player_hurtbox.root_entity)
        else {
            continue;
        };

        let Ok((mut enemy, mut enemy_stagger)) = q_enemies.get_mut(enemy_hitbox.root_entity) else {
            continue;
        };

        let HitboxType::Enemy(_attack) = enemy_hitbox.hitbox_type else {
            continue;
        };

        if player.state_machine.state() == DudeState::Parrying
            && player.state_machine.parry_state() == ParryState::Start
        {
            player.state_machine.set_parry_state(ParryState::Success);
            enemy.state_machine.set_new_state(DudeState::Staggering);
            enemy_stagger.set_stance_break();
            continue;
        }

        player.state_machine.set_new_state(DudeState::Staggering);
        player_stagger.set_player_stagger(enemy.state_machine.attack_direction());
    }
}

pub struct PlayerCollisionsPlugin;

impl Plugin for PlayerCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (enemy_hitbox_collisions,)
                .before(PlayerStateSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
