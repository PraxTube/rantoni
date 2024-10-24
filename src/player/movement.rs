use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::dude::{Attack, DudeState, JumpingState, Stagger};
use crate::GameState;

use super::input::PlayerInput;
use super::Player;

fn reset_velocity(mut q_player: Query<&mut Velocity, With<Player>>) {
    let Ok(mut velocity) = q_player.get_single_mut() else {
        return;
    };
    velocity.linvel = Vec2::ZERO;
}

fn move_player(player_input: Res<PlayerInput>, mut q_player: Query<(&Player, &mut Velocity)>) {
    let (player, mut velocity) = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let speed = match player.state_machine.state() {
        DudeState::Running => 250.0,
        _ => 0.0,
    };

    let direction = player_input.move_direction;
    velocity.linvel = direction * speed;
}

fn move_player_attacking(
    player_input: Res<PlayerInput>,
    mut q_players: Query<(&AnimationPlayer2D, &mut Velocity, &Player)>,
) {
    for (animator, mut velocity, player) in &mut q_players {
        if player.state_machine.attack_eq(Attack::Light1) {
            velocity.linvel = player.current_direction * 50.0;
        } else if player.state_machine.attack_eq(Attack::Light2) {
            velocity.linvel = player.current_direction * 250.0;
        } else if player.state_machine.attack_eq(Attack::Slide) {
            let Some(duration) = animator.duration() else {
                continue;
            };

            let x = animator.elapsed() / duration + 0.1;
            let multiplier = (1.0 - x.powi(2)).max(0.0);
            velocity.linvel = player.state_machine.attack_direction() * 400.0 * multiplier;
        } else if let DudeState::Jumping(jumping_state) = player.state_machine.state() {
            let Some(duration) = animator.duration() else {
                continue;
            };

            let x = animator.elapsed() / duration;
            let (speed, direction) = match jumping_state {
                JumpingState::Start => (
                    (1.0 - x.powi(2)).max(0.7) * 350.0,
                    player.state_machine.attack_direction(),
                ),
                JumpingState::RecoverIdle => (0.0, player_input.move_direction),
                JumpingState::RecoverMoving => (
                    (0.7 + x.powi(2)).min(1.0) * 250.0,
                    player_input.move_direction,
                ),
            };
            velocity.linvel = speed * direction;
        }
    }
}

fn move_player_staggering(mut q_players: Query<(&mut Velocity, &Player, &Stagger)>) {
    for (mut velocity, player, stagger) in &mut q_players {
        if player.state_machine.state() != DudeState::Staggering {
            continue;
        }

        velocity.linvel = stagger.direction * stagger.intensity;
    }
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, reset_velocity).add_systems(
            Update,
            (move_player, move_player_attacking, move_player_staggering)
                .chain()
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
