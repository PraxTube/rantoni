use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{input::PlayerInput, state::PlayerState, Player};

fn flip_player_sprite(player_input: Res<PlayerInput>, mut q_player: Query<(&Player, &mut Sprite)>) {
    let Ok((player, mut sprite)) = q_player.get_single_mut() else {
        return;
    };

    if player.state_machine.can_run() {
        if player_input.move_direction.x != 0.0 {
            sprite.flip_x = player_input.move_direction.x < 0.0;
        }
    } else {
        if player.punching_direction.x != 0.0 {
            sprite.flip_x = player.punching_direction.x < 0.0;
        }
    }
}

fn update_animations(
    assets: Res<GameAssets>,
    mut q_player: Query<(&Player, &mut AnimationPlayer2D)>,
) {
    let (player, mut animator) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let (animation, repeat) = match player.state_machine.state() {
        PlayerState::Idling => (assets.player_animations[0].clone(), true),
        PlayerState::Running => (assets.player_animations[1].clone(), true),
        PlayerState::Punching1 => (assets.player_animations[2].clone(), false),
        PlayerState::Punching1Recover => (assets.player_animations[3].clone(), false),
        PlayerState::Punching2 => (assets.player_animations[4].clone(), false),
        PlayerState::Punching2Recover => (assets.player_animations[5].clone(), false),
    };

    if repeat {
        animator.play(animation).repeat();
    } else {
        animator.play(animation);
    }
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flip_player_sprite, update_animations).run_if(resource_exists::<GameAssets>),
        );
    }
}
