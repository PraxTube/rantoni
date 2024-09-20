use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PlayerAttackState {
    #[default]
    Light1,
    Light2,
    Light3,
    Heavy1,
    Heavy2,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Attack {
    #[default]
    None,
    Light,
    Heavy,
}

pub struct AttackHandler {
    attack_state: PlayerAttackState,
    chained_attack: Attack,
    chainable: bool,
    chain_buffer_timer: Timer,
}

impl Default for AttackHandler {
    fn default() -> Self {
        Self {
            attack_state: PlayerAttackState::default(),
            chained_attack: Attack::default(),
            chainable: false,
            chain_buffer_timer: Timer::from_seconds(0.3, TimerMode::Once),
        }
    }
}

impl AttackHandler {
    pub fn attack_state(&self) -> PlayerAttackState {
        self.attack_state
    }

    pub fn set_attack_state(&mut self, attack_state: PlayerAttackState) {
        self.attack_state = attack_state;
    }

    pub fn chained_attack(&self) -> Attack {
        self.chained_attack
    }

    pub fn set_chained_attack(&mut self, chained_attack: Attack) {
        self.chained_attack = chained_attack;
    }

    pub fn chainable(&self) -> bool {
        self.chainable
    }

    pub fn set_chainable(&mut self, chainable: bool) {
        self.chainable = chainable;
        self.chain_buffer_timer.pause();
    }

    pub fn start_attack_chain_timer(&mut self) {
        self.chain_buffer_timer.unpause();
        self.chain_buffer_timer.reset();
    }

    pub fn handle_attack_chain_timer(&mut self, delta: Duration) {
        self.chain_buffer_timer.tick(delta);
        if self.chain_buffer_timer.just_finished() {
            self.chainable = false;
        }
    }
}
