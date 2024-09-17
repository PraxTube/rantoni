use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{input::PlayerInput, state::PlayerStateSystemSet, Player};

fn flip_player_sprite(player_input: Res<PlayerInput>, mut q_player: Query<(&Player, &mut Sprite)>) {
    let Ok((player, mut sprite)) = q_player.get_single_mut() else {
        return;
    };

    if player.state_machine.can_run() {
        if player_input.move_direction.x != 0.0 {
            sprite.flip_x = player_input.move_direction.x < 0.0;
        }
    } else if player.punching_direction.x != 0.0 {
        sprite.flip_x = player.punching_direction.x < 0.0;
    }
}

fn update_player_animation(
    assets: Res<GameAssets>,
    mut q_player: Query<(&Player, &mut AnimationPlayer2D)>,
) {
    let Ok((player, mut animator)) = q_player.get_single_mut() else {
        return;
    };

    let (animation, repeat) = player.state_machine.state_animation(&assets);
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
            (flip_player_sprite, update_player_animation)
                .after(PlayerStateSystemSet)
                .before(AnimationPlayer2DSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
