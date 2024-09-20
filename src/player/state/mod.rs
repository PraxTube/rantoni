mod attack;
mod state_machine;

pub use attack::{Attack, AttackHandler, PlayerAttackState};
pub use state_machine::PlayerStateMachine;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::player::{input::PlayerInput, Player};

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                reset_just_changed,
                transition_attack_state,
                transition_idle_state,
                transition_run_state,
            )
                .chain()
                .in_set(PlayerStateSystemSet),
        )
        .add_systems(
            Update,
            (
                update_aim_direction,
                start_attack_chain_timer,
                handle_attack_chain_timer,
            )
                .chain()
                .after(PlayerStateSystemSet),
        );
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Running,
    Attacking,
    Recovering,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerStateSystemSet;

fn transition_attack_state(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        let attack = if player_input.light_attack {
            Attack::Light
        } else if player_input.heavy_attack {
            Attack::Heavy
        } else {
            continue;
        };
        player.state_machine.transition_attack(attack);
    }
}

fn transition_run_state(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };
    if player.state_machine.just_changed() {
        return;
    }

    if !player.state_machine.can_run() {
        return;
    }

    if player_input.move_direction != Vec2::ZERO {
        if player.state_machine.state() != PlayerState::Running {
            player.state_machine.set_state(PlayerState::Running);
        }
    } else if player.state_machine.state() == PlayerState::Running {
        player.state_machine.set_state(PlayerState::Idling);
    };
}

fn transition_idle_state(mut q_player: Query<(&mut Player, &AnimationPlayer2D)>) {
    let Ok((mut player, animator)) = q_player.get_single_mut() else {
        return;
    };
    if player.state_machine.just_changed() {
        return;
    }

    if !animator.just_finished() {
        return;
    }

    match player.state_machine.state() {
        PlayerState::Idling | PlayerState::Running => {
            error!("should never happen! The current state's animation should be repeating forever and never finish")
        }
        PlayerState::Attacking => {
            player.state_machine.transition_chain_attack();
        }
        PlayerState::Recovering => {
            player.state_machine.set_state(PlayerState::Idling);
        }
    };
}

fn reset_just_changed(mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    player.state_machine.set_just_changed(false);
}

fn update_aim_direction(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player_input.aim_direction == Vec2::ZERO {
            continue;
        }

        if player.state_machine.just_changed() {
            player.aim_direction = player_input.aim_direction;
        }
    }
}

fn start_attack_chain_timer(mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if !player.state_machine.just_changed() {
            continue;
        }

        if player.state_machine.previous_state() == PlayerState::Attacking
            && player.state_machine.state() != PlayerState::Attacking
        {
            player.state_machine.start_attack_chain_timer();
        }
    }
}

fn handle_attack_chain_timer(time: Res<Time>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        player.state_machine.handle_attack_chain_timer(time.delta());
    }
}
