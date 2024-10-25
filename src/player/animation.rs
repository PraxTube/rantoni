use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor};
use bevy_trickfilm::prelude::*;

use crate::{
    dude::{dude_state_animation, DudeState, JumpingState},
    GameAssets,
};

use super::{input::PlayerInput, state::PlayerStateSystemSet, Player};

const JUMP_HEIGHT: f32 = 30.0 / 100.0;

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
        player.state_machine.stagger_state(),
        player.state_machine.parry_state(),
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

fn animate_sprite_jumping(mut q_players: Query<(&mut Sprite, &AnimationPlayer2D, &Player)>) {
    for (mut sprite, animator, player) in &mut q_players {
        if player.state_machine.state() != DudeState::Jumping(JumpingState::Start) {
            sprite.anchor = Anchor::Center;
            continue;
        }

        if let Some(duration) = animator.duration() {
            let x = animator.elapsed() / duration * PI;
            let offset = JUMP_HEIGHT * x.sin();
            sprite.anchor = Anchor::Custom(Vec2::new(0.0, -offset));
        }
    }
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_current_direction,
                update_player_animation,
                animate_sprite_jumping,
            )
                .chain()
                .after(PlayerStateSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
