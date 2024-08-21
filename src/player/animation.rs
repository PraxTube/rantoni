use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{input::PlayerInput, state::trigger_player_changed_state, Player};

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

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flip_player_sprite,)
                .before(AnimationPlayer2DSystemSet)
                .after(trigger_player_changed_state)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
