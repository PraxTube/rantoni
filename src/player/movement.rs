use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

use super::input::PlayerInput;
use super::state::PlayerState;
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
        super::state::PlayerState::Idling => 0.0,
        super::state::PlayerState::Running => 300.0,
        super::state::PlayerState::Punching1 => 0.0,
        super::state::PlayerState::Punching1Recover => 0.0,
        super::state::PlayerState::Punching2 => 0.0,
        super::state::PlayerState::Punching2Recover => 0.0,
    };

    let direction = player_input.move_direction;
    velocity.linvel = direction * speed;
}

fn move_player_punching(mut q_player: Query<(&Player, &mut Velocity)>) {
    let Ok((player, mut velocity)) = q_player.get_single_mut() else {
        return;
    };

    if player.state_machine.state() == PlayerState::Punching1 {
        velocity.linvel = player.punching_direction * 50.0;
    } else if player.state_machine.state() == PlayerState::Punching2 {
        velocity.linvel = player.punching_direction * 250.0;
    }
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, reset_velocity).add_systems(
            Update,
            (move_player, move_player_punching)
                .chain()
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
