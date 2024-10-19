use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::{dude_state_animation, StaggerState},
    GameAssets,
};

use super::{input::PlayerInput, state::PlayerStateSystemSet, Player};

fn update_current_direction(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    if player_input.move_direction == Vec2::ZERO {
        return;
    }

    player.current_direction = player_input.move_direction;
}

fn update_player_animation(
    assets: Res<GameAssets>,
    mut q_player: Query<(&mut Player, &mut Handle<Image>, &mut AnimationPlayer2D)>,
) {
    let Ok((mut player, mut player_texture, mut animator)) = q_player.get_single_mut() else {
        return;
    };

    let (texture, animation, repeat, animation_state) = dude_state_animation(
        &assets,
        player.state_machine.state(),
        player.state_machine.attack(),
        StaggerState::default(),
        player.current_direction,
    );

    if &animation == animator.animation_clip() {
        return;
    }
    if !repeat && animation_state == player.state_machine.animation_state() {
        return;
    }
    player.state_machine.set_animation_state(animation_state);

    if repeat {
        animator.play(animation).repeat();
    } else {
        animator.play(animation);
        // TODO: Handle this better, maybe you want to put this outside the if else statement?
        let direction = player.current_direction;
        player.state_machine.set_attack_direction(direction);
    }

    *player_texture = texture;
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_current_direction, update_player_animation)
                .chain()
                .after(PlayerStateSystemSet)
                .before(AnimationPlayer2DSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
