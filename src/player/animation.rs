use bevy::{prelude::*, sprite::Anchor};
use bevy_trickfilm::prelude::*;

use crate::{
    assets::events::SpawnHitboxEvent,
    dude::{dude_state_animation_player, Attack, DudeState},
    GameAssets,
};

use super::{input::PlayerInput, state::PlayerStateSystemSet, Player};

fn update_current_directions(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player_input.move_direction == Vec2::ZERO {
            continue;
        }
        if player.state_machine.state() == DudeState::Attacking
            || player.state_machine.state() == DudeState::Recovering
        {
            continue;
        }

        player.current_direction = player_input.move_direction;
    }
}

fn update_player_animation(
    assets: Res<GameAssets>,
    mut q_player: Query<(&mut Player, &mut Handle<Image>, &mut AnimationPlayer2D)>,
) {
    let Ok((mut player, mut player_texture, mut animator)) = q_player.get_single_mut() else {
        return;
    };

    if player.state_machine.state() == DudeState::Dashing {
        return;
    }

    let direction = match player.state_machine.state() {
        DudeState::Idling | DudeState::Running | DudeState::Staggering | DudeState::Parrying(_) => {
            player.current_direction
        }
        DudeState::Attacking | DudeState::Recovering => player.state_machine.attack_direction(),
        DudeState::Dashing => panic!("should never happen! Dashing should not use this function"),
        DudeState::Dying => panic!("should never happen! Dying should not use this function"),
        DudeState::Stalking => panic!("player must never be in stalking, refactor this anyways"),
    };

    let (texture, animation, repeat, animation_state) = dude_state_animation_player(
        &assets,
        player.state_machine.state(),
        player.state_machine.attack(),
        player.state_machine.stagger_state(),
        direction,
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
    }
    *player_texture = texture;
}

fn toggle_dashing_players_visibility(mut q_players: Query<(&mut Sprite, &Player)>) {
    for (mut sprite, player) in &mut q_players {
        let alpha = if player.state_machine.state() == DudeState::Dashing {
            0.0
        } else {
            1.0
        };

        sprite.color.set_alpha(alpha);
    }
}

fn animate_sprite_jumping(mut q_players: Query<(&mut Sprite, &Player)>) {
    for (mut sprite, player) in &mut q_players {
        if player.state_machine.attack_eq(Attack::Dropkick)
            || player.state_machine.attack_eq(Attack::Hammerfist)
        {
            let offset = player.state_machine.sprite_y_offset();
            sprite.anchor = Anchor::Custom(Vec2::new(0.0, -offset));
        } else {
            sprite.anchor = Anchor::Center;
        }
    }
}

fn disable_can_move_during_attack(
    mut q_players: Query<&mut Player>,
    mut ev_spawn_hitbox: EventReader<SpawnHitboxEvent>,
) {
    for ev in ev_spawn_hitbox.read() {
        let Ok(mut player) = q_players.get_mut(*ev.target) else {
            continue;
        };

        player.state_machine.disable_can_move_during_attack();
    }
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_current_directions,
                update_player_animation,
                toggle_dashing_players_visibility,
                animate_sprite_jumping,
                disable_can_move_during_attack,
            )
                .chain()
                .after(PlayerStateSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
