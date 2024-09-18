use bevy::prelude::*;
use bevy_rancic::prelude::COLLISION_GROUPS_NONE;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::world::collisions::{Hitbox, HitboxType};

use super::{state::PlayerState, Player, PlayerHitboxRoot, PlayerStateSystemSet};

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

fn flip_hitbox_positions(
    q_players: Query<(Entity, &Player)>,
    q_player_root_hitboxes: Query<(&Children, &PlayerHitboxRoot)>,
    mut q_hitboxes: Query<(&mut Transform, &Hitbox)>,
) {
    for (player_entity, player) in &q_players {
        for (children, root_hitbox) in &q_player_root_hitboxes {
            if root_hitbox.root_entity != player_entity {
                continue;
            }

            for child in children {
                let Ok((mut transform, hitbox)) = q_hitboxes.get_mut(*child) else {
                    continue;
                };

                if hitbox.horizontal {
                    let pos = if player.aim_direction.x < 0.0 {
                        Vec2::new(-hitbox.offset.x, hitbox.offset.y)
                    } else {
                        hitbox.offset
                    };
                    transform.translation = pos.extend(0.0);
                }
            }
        }
    }
}

pub struct PlayerCollisionsPlugin;

impl Plugin for PlayerCollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flip_hitbox_positions, disable_all_hitboxes, toggle_hitboxes)
                .chain()
                .after(PlayerStateSystemSet)
                .after(AnimationPlayer2DSystemSet),
        );
    }
}
