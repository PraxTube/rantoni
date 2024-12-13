use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::dude::{Attack, DudeState};
use crate::GameState;

use super::input::PlayerInput;
use super::Player;

fn reset_velocity(mut q_player: Query<&mut Velocity, With<Player>>) {
    let Ok(mut velocity) = q_player.get_single_mut() else {
        return;
    };
    velocity.linvel = Vec2::ZERO;
}

fn move_running(player_input: Res<PlayerInput>, mut q_player: Query<(&Player, &mut Velocity)>) {
    let (player, mut velocity) = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let speed = match player.state_machine.state() {
        DudeState::Running => 350.0,
        _ => 0.0,
    };

    let direction = player_input.move_direction;
    velocity.linvel = direction * speed;
}

fn move_attacking(mut q_players: Query<(&mut Velocity, &Player)>) {
    // TODO: Refactor this, intuitively I expeted to find these values and stuff in
    // `dude/attack.rs`, with each attack having some kind of information about the movement, it
    // would be a little tricky to pull that off as the direction isn't always the same, but still
    // this isn't _super_ nice, although I don't know how much that matters as we will have
    // different attacks for each character anyways, but still will need to find a way to at least
    // have all of those informations in one place.
    //
    // Actually, I probably want to have a simple ron file or something else that is human readable
    // and just read the values in from there. This would also allow others to test different
    // values and just make things more user friendly in general.
    for (mut velocity, player) in &mut q_players {
        if player.state_machine.state() != DudeState::Attacking {
            continue;
        }

        let can_move = if player.state_machine.can_move_during_attack() {
            1.0
        } else {
            0.0
        };

        let speed = match player.state_machine.attack() {
            Attack::Light1 => can_move * 130.0,
            Attack::Light2 => can_move * 325.0,
            Attack::Light3 => can_move * 300.0,
            Attack::Heavy1 => can_move * 250.0,
            Attack::Heavy2 => can_move * 75.0,
            Attack::Heavy3 => can_move * 300.0,
            Attack::Dropkick | Attack::Hammerfist => {
                player.state_machine.jump_attack_speed_multiplier() * 450.0
            }
        };
        velocity.linvel = player.state_machine.attack_direction() * speed;
    }
}

fn move_dashing(mut q_players: Query<(&mut Velocity, &Player)>) {
    for (mut velocity, player) in &mut q_players {
        if player.state_machine.state() == DudeState::Dashing {
            velocity.linvel = player.state_machine.attack_direction() * 1000.0;
        }
    }
}

fn move_staggering(mut q_players: Query<(&mut Velocity, &Player)>) {
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

fn move_dying(mut q_players: Query<(&mut Velocity, &AnimationPlayer2D, &Player)>) {
    for (mut velocity, animator, player) in &mut q_players {
        if player.state_machine.state() == DudeState::Dying && !animator.finished() {
            velocity.linvel = -player.current_direction * 100.0;
        }
    }
}

fn debug(q_players: Query<&Transform, With<Player>>) {
    for transform in &q_players {
        info!("{}", transform.translation);
    }
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, reset_velocity).add_systems(
            Update,
            (
                move_running,
                move_attacking,
                move_staggering,
                move_dashing,
                move_dying,
                debug,
            )
                .chain()
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
