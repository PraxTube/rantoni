use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::dude::{Attack, DudeState, JumpingState};
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

fn move_player_attacking(mut q_players: Query<(&AnimationPlayer2D, &mut Velocity, &Player)>) {
    // TODO: Refactor this, intuitively I expeted to find these values and stuff in
    // `dude/attack.rs`, with each attack having some kind of information about the movement, it
    // would be a little tricky to pull that off as the direction isn't always the same, but still
    // this isn't _super_ nice, although I don't know how much that matters as we will have
    // different attacks for each character anyways, but still will need to find a way to at least
    // have all of those informations in one place.
    for (animator, mut velocity, player) in &mut q_players {
        if player.state_machine.state() != DudeState::Attacking {
            continue;
        }

        let can_move = if player.state_machine.can_move_during_attack() {
            1.0
        } else {
            0.0
        };

        match player.state_machine.attack() {
            Attack::Light1 => velocity.linvel = player.current_direction * 100.0 * can_move,
            Attack::Light2 => velocity.linvel = player.current_direction * 250.0 * can_move,
            Attack::Light3 => velocity.linvel = player.current_direction * 200.0 * can_move,
            Attack::Heavy1 => velocity.linvel = player.current_direction * 200.0 * can_move,
            Attack::Heavy2 => velocity.linvel = player.current_direction * 50.0 * can_move,
            Attack::Heavy3 => velocity.linvel = player.current_direction * 250.0 * can_move,
            Attack::Slide => {
                let Some(duration) = animator.duration() else {
                    continue;
                };

                let x = animator.elapsed() / duration;
                let multiplier = (1.0 - x.powi(2)).max(0.0);
                velocity.linvel = player.state_machine.attack_direction() * 400.0 * multiplier;
            }
            Attack::Dropkick | Attack::Hammerfist => {
                velocity.linvel = player
                    .state_machine
                    .jumping_linvel(player.state_machine.attack_direction());
            }
        }
    }
}

fn move_player_jumping(
    player_input: Res<PlayerInput>,
    mut q_players: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in &mut q_players {
        if let DudeState::Jumping(jumping_state) = player.state_machine.state() {
            let direction = match jumping_state {
                JumpingState::Start => player.state_machine.attack_direction(),
                JumpingState::RecoverIdle => Vec2::ZERO,
                JumpingState::RecoverMoving => player_input.move_direction,
            };
            velocity.linvel = player.state_machine.jumping_linvel(direction);
        }
    }
}

fn move_player_staggering(mut q_players: Query<(&mut Velocity, &Player)>) {
    for (mut velocity, player) in &mut q_players {
        if player.state_machine.state() != DudeState::Staggering {
            continue;
        }
        if player.state_machine.stagger_state().is_recovering() {
            continue;
        }

        velocity.linvel = player.state_machine.stagger_linvel();
    }
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, reset_velocity).add_systems(
            Update,
            (
                move_player,
                move_player_attacking,
                move_player_jumping,
                move_player_staggering,
            )
                .chain()
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
