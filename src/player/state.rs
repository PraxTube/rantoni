use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use super::{input::PlayerInput, Player};

#[derive(Event)]
pub struct PlayerChangedState {
    state: PlayerState,
    #[allow(dead_code)]
    previous_state: PlayerState,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Running,
    Punching1,
    Punching1Recover,
    Punching2,
    Punching2Recover,
}

#[derive(Component, Default)]
pub struct PlayerStateMachine {
    state: PlayerState,
    /// This is used to queue multiple non repeating states.
    /// For example Punching1 -> Punchin2 etc.
    /// This gets triggered as soon as the current state is done.
    /// And instead of going into Idle the state machine will switch to this state.
    queued_state: Option<PlayerState>,
    previous_state: PlayerState,
    just_changed_state: bool,
}

impl PlayerStateMachine {
    pub fn can_run(&self) -> bool {
        self.state == PlayerState::Idling
            || self.state == PlayerState::Running
            || self.state == PlayerState::Punching1Recover
            || self.state == PlayerState::Punching2Recover
    }

    fn can_punch(&self) -> bool {
        self.can_run() || self.state == PlayerState::Punching1
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }

    pub fn set_state(&mut self, state: PlayerState) {
        self.previous_state = self.state;
        self.just_changed_state = true;
        self.state = state;
    }

    pub fn take_queued_state(&mut self) -> Option<PlayerState> {
        self.queued_state.take()
    }

    pub fn set_queued_state(&mut self, state: PlayerState) {
        self.queued_state = Some(state);
    }
}

fn transition_run_state(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    if !player.state_machine.can_run() {
        return;
    }

    if player_input.move_direction != Vec2::ZERO {
        player.state_machine.set_state(PlayerState::Running);
    } else if player.state_machine.state() == PlayerState::Running {
        player.state_machine.set_state(PlayerState::Idling);
    };
}

fn transition_punch_state(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    if !player.state_machine.can_punch() {
        return;
    }

    if player_input.punched {
        player.punching_direction = player_input.aim_direction;
        if player.state_machine.state() == PlayerState::Punching1 {
            player
                .state_machine
                .set_queued_state(PlayerState::Punching2);
        } else {
            player.state_machine.state = PlayerState::Punching1;
            player
                .state_machine
                .set_queued_state(PlayerState::Punching1Recover);
        }
    }
}

fn transition_idle_state(mut q_player: Query<(&mut Player, &AnimationPlayer2D)>) {
    let Ok((mut player, animator)) = q_player.get_single_mut() else {
        return;
    };

    if animator.just_finished() {
        let state = player
            .state_machine
            .take_queued_state()
            .unwrap_or(PlayerState::Idling);
        player.state_machine.set_state(state);
    }
}

fn set_punching2_recover_queue_state(
    mut q_player: Query<&mut Player>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };
    for ev in ev_player_changed_state.read() {
        if ev.state == PlayerState::Punching2 {
            player
                .state_machine
                .set_queued_state(PlayerState::Punching2Recover);
        }
    }
}

fn trigger_player_changed_state(
    mut q_player: Query<&mut Player>,
    mut ev_player_changed_state: EventWriter<PlayerChangedState>,
) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    if player.state_machine.just_changed_state {
        player.state_machine.just_changed_state = false;
        ev_player_changed_state.send(PlayerChangedState {
            state: player.state_machine.state,
            previous_state: player.state_machine.previous_state,
        });
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerChangedState>().add_systems(
            Update,
            (
                transition_run_state,
                transition_punch_state,
                transition_idle_state,
                set_punching2_recover_queue_state,
                trigger_player_changed_state,
            ),
        );
    }
}
