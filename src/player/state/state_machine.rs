use std::time::Duration;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{Attack, AttackHandler, PlayerAttackState, PlayerState};

#[derive(Component, Default)]
pub struct PlayerStateMachine {
    just_changed: bool,
    state: PlayerState,
    previous_state: PlayerState,
    attack_handler: AttackHandler,
}

impl PlayerStateMachine {
    pub fn can_run(&self) -> bool {
        self.state == PlayerState::Idling
            || self.state == PlayerState::Running
            || self.state == PlayerState::Recovering
    }

    pub fn can_punch(&self) -> bool {
        self.can_run() || self.state == PlayerState::Attacking
    }

    pub fn previous_state(&self) -> PlayerState {
        self.previous_state
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }

    pub fn set_state(&mut self, state: PlayerState) {
        if self.just_changed {
            error!("Trying to set state even though it was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }

        self.set_just_changed(true);
        self.previous_state = self.state;
        self.state = state;
    }

    pub fn attack_state(&self) -> PlayerAttackState {
        self.attack_handler.attack_state()
    }

    pub fn attack_state_eq(&self, attack_state: PlayerAttackState) -> bool {
        self.state == PlayerState::Attacking && self.attack_state() == attack_state
    }

    pub fn set_attack_state(&mut self, attack_state: PlayerAttackState) {
        if self.just_changed {
            error!("Trying to set state even though it was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }
        self.set_state(PlayerState::Attacking);
        self.attack_handler.set_attack_state(attack_state);
        self.attack_handler.set_chainable(true);
    }

    pub fn chained_attack(&self) -> Attack {
        self.attack_handler.chained_attack()
    }

    pub fn set_chained_attack(&mut self, attack: Attack) {
        self.attack_handler.set_chained_attack(attack);
    }

    pub fn just_changed(&self) -> bool {
        self.just_changed
    }

    pub fn set_just_changed(&mut self, just_changed: bool) {
        self.just_changed = just_changed;
    }

    pub fn start_attack_chain_timer(&mut self) {
        self.attack_handler.start_attack_chain_timer();
    }

    pub fn handle_attack_chain_timer(&mut self, delta: Duration) {
        self.attack_handler.handle_attack_chain_timer(delta);
    }

    pub fn default_attack(&mut self, attack: Attack) {
        match attack {
            Attack::None => {}
            Attack::Light => self.set_attack_state(PlayerAttackState::Light1),
            Attack::Heavy => self.set_attack_state(PlayerAttackState::Heavy1),
        }
    }

    pub fn combo_attack(&self, attack: Attack) -> Option<PlayerAttackState> {
        match self.attack_state() {
            PlayerAttackState::Light1 => match attack {
                Attack::None => panic!("should never happen!"),
                Attack::Light => Some(PlayerAttackState::Light2),
                Attack::Heavy => Some(PlayerAttackState::Heavy1),
            },
            PlayerAttackState::Light2 => match attack {
                Attack::None => panic!("should never happen!"),
                Attack::Light => Some(PlayerAttackState::Light3),
                Attack::Heavy => None,
            },
            PlayerAttackState::Light3 => None,
            PlayerAttackState::Heavy1 => match attack {
                Attack::None => panic!("should never happen!"),
                Attack::Light => Some(PlayerAttackState::Light2),
                Attack::Heavy => Some(PlayerAttackState::Heavy2),
            },
            PlayerAttackState::Heavy2 => None,
        }
    }

    pub fn transition_chain_attack(&mut self) {
        if self.chained_attack() == Attack::None {
            self.set_state(PlayerState::Recovering);
            return;
        }

        match self.combo_attack(self.chained_attack()) {
            Some(attack_state) => self.set_attack_state(attack_state),
            None => self.set_state(PlayerState::Recovering),
        }
        self.set_chained_attack(Attack::None);
    }

    pub fn transition_attack(&mut self, attack: Attack) {
        if self.just_changed() {
            return;
        }
        if !self.can_punch() {
            return;
        }

        if self.state() == PlayerState::Attacking {
            assert_ne!(attack, Attack::None);
            self.set_chained_attack(attack);
        } else if self.attack_handler.chainable() {
            match self.combo_attack(attack) {
                Some(attack_state) => self.set_attack_state(attack_state),
                None => self.default_attack(attack),
            }
        } else {
            self.default_attack(attack);
        }
    }

    pub fn state_animation(&self, assets: &Res<GameAssets>) -> (Handle<AnimationClip2D>, bool) {
        match self.state {
            PlayerState::Idling => (assets.player_animations[0].clone(), true),
            PlayerState::Running => (assets.player_animations[1].clone(), true),
            PlayerState::Attacking => match self.attack_state() {
                PlayerAttackState::Light1 => (assets.player_animations[2].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[4].clone(), false),
                PlayerAttackState::Light3 => (assets.player_animations[11].clone(), false),
                PlayerAttackState::Heavy1 => (assets.player_animations[7].clone(), false),
                PlayerAttackState::Heavy2 => (assets.player_animations[9].clone(), false),
            },
            PlayerState::Recovering => match self.attack_state() {
                PlayerAttackState::Light1 => (assets.player_animations[3].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[5].clone(), false),
                PlayerAttackState::Light3 => (assets.player_animations[12].clone(), false),
                PlayerAttackState::Heavy1 => (assets.player_animations[8].clone(), false),
                PlayerAttackState::Heavy2 => (assets.player_animations[10].clone(), false),
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
            PlayerState::Attacking => match self.attack_state() {
                PlayerAttackState::Light1 => (0, 1),
                PlayerAttackState::Light2 => (0, 1),
                PlayerAttackState::Light3 => (1, 2),
                PlayerAttackState::Heavy1 => (1, 2),
                PlayerAttackState::Heavy2 => (1, 2),
            },
            PlayerState::Recovering => {
                error!("should never happen! recover doesn't have any hitbox frames");
                (0, 0)
            }
        }
    }
}
