use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    state::{dude_state_animation, StaggerState},
    GameAssets,
};

use super::{input::PlayerInput, state::PlayerStateSystemSet, Player};

fn update_player_animation(
    assets: Res<GameAssets>,
    player_input: Res<PlayerInput>,
    mut q_player: Query<(&Player, &mut Handle<Image>, &mut AnimationPlayer2D)>,
) {
    let Ok((player, mut player_texture, mut animator)) = q_player.get_single_mut() else {
        return;
    };

    let (texture, animation, repeat) = dude_state_animation(
        &assets,
        player.state_machine.state(),
        player.state_machine.attack(),
        StaggerState::default(),
        player_input.move_direction,
    );

    if &animation == animator.animation_clip() {
        return;
    }

    if repeat {
        animator.play(animation).repeat();
    } else {
        animator.play(animation);
    }

    *player_texture = texture;
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_player_animation,)
                .after(PlayerStateSystemSet)
                .before(AnimationPlayer2DSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
