use std::time::Duration;

use bevy::prelude::*;

use crate::{
    state::{Attack, AttackForm},
    world::collisions::HitboxDirection,
};

pub struct AttackHandler {
    attack: Attack,
    attack_direction: HitboxDirection,
    chained_attack: AttackForm,
    chainable: bool,
    chain_buffer_timer: Timer,
}

impl Default for AttackHandler {
    fn default() -> Self {
        Self {
            attack: Attack::default(),
            attack_direction: HitboxDirection::Top,
            chained_attack: AttackForm::default(),
            chainable: false,
            chain_buffer_timer: Timer::from_seconds(0.3, TimerMode::Once),
        }
    }
}

impl AttackHandler {
    pub fn attack(&self) -> Attack {
        self.attack
    }

    pub fn set_attack(&mut self, attack: Attack) {
        self.attack = attack;
    }

    pub fn attack_direction(&self) -> HitboxDirection {
        self.attack_direction
    }

    pub fn set_attack_direction(&mut self, direction: Vec2) {
        self.attack_direction = HitboxDirection::from(direction);
    }

    pub fn chained_attack(&self) -> AttackForm {
        self.chained_attack
    }

    pub fn set_chained_attack(&mut self, chained_attack: AttackForm) {
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
