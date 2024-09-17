use bevy::prelude::*;
use bevy_rancic::prelude::COLLISION_GROUPS_NONE;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::world::collisions::{Hitbox, HitboxType};

use super::{state::PlayerState, Player, PlayerStateSystemSet};

fn toggle_hitboxes(
    q_players: Query<(Entity, &AnimationPlayer2D, &Player)>,
    mut q_hitboxes: Query<(&mut CollisionGroups, &Hitbox)>,
) {
    for (player_entity, animator, player) in &q_players {
        if player.state_machine.state() != PlayerState::Attacking {
            continue;
        }

        for (mut collisions_groups, hitbox) in &mut q_hitboxes {
            if hitbox.root_entity != player_entity {
                continue;
            }

            if hitbox.hitbox_type == HitboxType::Player(player.state_machine.attack_state()) {
                let (start_frame, end_frame) = player.state_machine.state_hitbox_frames();

                if animator.frame() == start_frame {
                    *collisions_groups = hitbox.collision_groups();
                } else if animator.frame() == end_frame {
                    *collisions_groups = COLLISION_GROUPS_NONE;
                }
            }
        }
    }
}

fn disable_all_hitboxes(
    q_players: Query<(Entity, &Player)>,
    mut q_hitboxes: Query<(&mut CollisionGroups, &Hitbox)>,
) {
    for (player_entity, player) in &q_players {
        if !player.state_machine.just_changed() {
            continue;
        }

        for (mut collisions_groups, hitbox) in &mut q_hitboxes {
            if hitbox.root_entity == player_entity {
                *collisions_groups = COLLISION_GROUPS_NONE;
            }
        }
    }
}

pub struct PlayerCollisionsPlugin;

impl Plugin for PlayerCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (disable_all_hitboxes, toggle_hitboxes)
                .chain()
                .after(PlayerStateSystemSet)
                .after(AnimationPlayer2DSystemSet),
        );
    }
}
