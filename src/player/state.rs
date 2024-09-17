use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{input::PlayerInput, Player};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Running,
    Attacking,
    Recovering,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerAttackState {
    #[default]
    Light1,
    Light2,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ChainAttack {
    #[default]
    None,
    Light,
    Heavy,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerStateSystemSet;

#[derive(Component, Default)]
pub struct PlayerStateMachine {
    state: PlayerState,
    attack_state: PlayerAttackState,
    previous_state: PlayerState,
    chain_attack: ChainAttack,
    just_changed: bool,
}

impl PlayerStateMachine {
    pub fn can_run(&self) -> bool {
        self.state == PlayerState::Idling
            || self.state == PlayerState::Running
            || self.state == PlayerState::Recovering
    }

    pub fn can_punch(&self) -> bool {
        self.can_run()
            || self.state == PlayerState::Attacking
                && self.attack_state == PlayerAttackState::Light1
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }

    fn set_state(&mut self, state: PlayerState) {
        if self.just_changed {
            error!("Trying to set state even though it was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }

        self.set_just_changed(true);
        self.previous_state = self.state;
        self.state = state;
    }

    pub fn attack_state(&self) -> PlayerAttackState {
        self.attack_state
    }

    pub fn attack_state_eq(&self, attack_state: PlayerAttackState) -> bool {
        self.state == PlayerState::Attacking && self.attack_state == attack_state
    }

    fn set_attack_state(&mut self, attack_state: PlayerAttackState) {
        if self.just_changed {
            error!("Trying to set state even though it was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }
        self.set_state(PlayerState::Attacking);
        self.attack_state = attack_state;
    }

    pub fn chain_attack(&self) -> ChainAttack {
        self.chain_attack
    }

    fn set_chain_attack(&mut self, chain_attack: ChainAttack) {
        self.chain_attack = chain_attack;
    }

    fn just_changed(&self) -> bool {
        self.just_changed
    }

    fn set_just_changed(&mut self, just_changed: bool) {
        self.just_changed = just_changed;
    }

    pub fn state_animation(&self, assets: &Res<GameAssets>) -> (Handle<AnimationClip2D>, bool) {
        match self.state {
            PlayerState::Idling => (assets.player_animations[0].clone(), true),
            PlayerState::Running => (assets.player_animations[1].clone(), true),
            PlayerState::Attacking => match self.attack_state {
                PlayerAttackState::Light1 => (assets.player_animations[2].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[4].clone(), false),
            },
            PlayerState::Recovering => match self.attack_state {
                PlayerAttackState::Light1 => (assets.player_animations[3].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[5].clone(), false),
            },
        }
    }
}

fn transition_punch_state(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };
    if player.state_machine.just_changed() {
        return;
    }

    if !player.state_machine.can_punch() {
        return;
    }

    if player_input.punched {
        player.punching_direction = player_input.aim_direction;
        if player
            .state_machine
            .attack_state_eq(PlayerAttackState::Light1)
        {
            player.state_machine.set_chain_attack(ChainAttack::Light);
        } else {
            player
                .state_machine
                .set_attack_state(PlayerAttackState::Light1);
        }
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
            if player.state_machine.chain_attack() == ChainAttack::None {
                player.state_machine.set_state(PlayerState::Recovering);
            } else {
                player.state_machine.set_chain_attack(ChainAttack::None);
                match player.state_machine.attack_state() {
                    PlayerAttackState::Light1 => player
                        .state_machine
                        .set_attack_state(PlayerAttackState::Light2),
                    PlayerAttackState::Light2 => {
                        player.state_machine.set_state(PlayerState::Recovering)
                    }
                };
            }
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

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                reset_just_changed,
                transition_punch_state,
                transition_idle_state,
                transition_run_state,
            )
                .chain()
                .in_set(PlayerStateSystemSet),
        );
    }
}
