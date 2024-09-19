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

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PlayerAttackState {
    #[default]
    Light1,
    Light2,
    Heavy1,
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

    pub fn just_changed(&self) -> bool {
        self.just_changed
    }

    fn set_just_changed(&mut self, just_changed: bool) {
        self.just_changed = just_changed;
    }

    fn transition_chain_attack(&mut self) {
        if self.chain_attack() == ChainAttack::None {
            self.set_state(PlayerState::Recovering);
            return;
        }

        match self.attack_state() {
            PlayerAttackState::Light1 => match self.chain_attack() {
                ChainAttack::None => panic!("should never happen!"),
                ChainAttack::Light => self.set_attack_state(PlayerAttackState::Light2),
                ChainAttack::Heavy => self.set_attack_state(PlayerAttackState::Heavy1),
            },
            PlayerAttackState::Light2 => self.set_state(PlayerState::Recovering),
            PlayerAttackState::Heavy1 => self.set_state(PlayerState::Recovering),
        };
        self.set_chain_attack(ChainAttack::None);
    }

    pub fn state_animation(&self, assets: &Res<GameAssets>) -> (Handle<AnimationClip2D>, bool) {
        match self.state {
            PlayerState::Idling => (assets.player_animations[0].clone(), true),
            PlayerState::Running => (assets.player_animations[1].clone(), true),
            PlayerState::Attacking => match self.attack_state {
                PlayerAttackState::Light1 => (assets.player_animations[2].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[4].clone(), false),
                PlayerAttackState::Heavy1 => (assets.player_animations[7].clone(), false),
            },
            PlayerState::Recovering => match self.attack_state {
                PlayerAttackState::Light1 => (assets.player_animations[3].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[5].clone(), false),
                PlayerAttackState::Heavy1 => (assets.player_animations[8].clone(), false),
            },
        }
    }

    pub fn state_hitbox_frames(&self) -> (usize, usize) {
        match self.state {
            PlayerState::Idling => {
                error!("should never happen! idle doesn't have any hitbox frames");
                (0, 0)
            }
            PlayerState::Running => {
                error!("should never happen! run doesn't have any hitbox frames");
                (0, 0)
            }
            PlayerState::Attacking => match self.attack_state {
                PlayerAttackState::Light1 => (0, 1),
                PlayerAttackState::Light2 => (0, 1),
                PlayerAttackState::Heavy1 => (1, 2),
            },
            PlayerState::Recovering => {
                error!("should never happen! recover doesn't have any hitbox frames");
                (0, 0)
            }
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

    if player_input.aim_direction != Vec2::ZERO {
        player.aim_direction = player_input.aim_direction;
    }

    if player_input.light_attack {
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
    } else if player_input.heavy_attack {
        if player
            .state_machine
            .attack_state_eq(PlayerAttackState::Light1)
        {
            player.state_machine.set_chain_attack(ChainAttack::Heavy);
        } else {
            player
                .state_machine
                .set_attack_state(PlayerAttackState::Heavy1);
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
