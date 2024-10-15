use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::state::{dude_state_hitbox_frames, DudeState};
use crate::world::collisions::{Hitbox, HitboxType};

use super::{Player, PlayerStateSystemSet};

fn toggle_hitboxes(
    q_players: Query<(Entity, &AnimationPlayer2D, &Player)>,
    mut q_hitboxes: Query<(&mut CollisionGroups, &mut ColliderDebugColor, &Hitbox)>,
) {
    for (player_entity, animator, player) in &q_players {
        if player.state_machine.state() != DudeState::Attacking {
            continue;
        }

        for (mut collisions_groups, mut collider_color, hitbox) in &mut q_hitboxes {
            if hitbox.root_entity != player_entity {
                continue;
            }
            if hitbox.hitbox_direction != player.state_machine.attack_direction() {
                continue;
            }
            if hitbox.hitbox_type != HitboxType::Player(player.state_machine.attack()) {
                continue;
            }

            let (start_frame, end_frame) = dude_state_hitbox_frames(
                player.state_machine.state(),
                player.state_machine.attack(),
            );
            if animator.frame() == start_frame {
                *collisions_groups = hitbox.collision_groups();
                *collider_color = COLLIDER_COLOR_WHITE;
            } else if animator.frame() == end_frame {
                *collisions_groups = COLLISION_GROUPS_NONE;
                *collider_color = COLLIDER_COLOR_BLACK;
            }
        }
    }
}

fn disable_all_hitboxes(
    q_players: Query<(Entity, &Player)>,
    mut q_hitboxes: Query<(&mut CollisionGroups, &mut ColliderDebugColor, &Hitbox)>,
) {
    for (player_entity, player) in &q_players {
        if !player.state_machine.just_changed() {
            continue;
        }

        for (mut collisions_groups, mut collider_color, hitbox) in &mut q_hitboxes {
            if hitbox.root_entity == player_entity {
                *collisions_groups = COLLISION_GROUPS_NONE;
                *collider_color = COLLIDER_COLOR_BLACK;
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
