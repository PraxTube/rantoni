use bevy::prelude::*;

use crate::state::{Attack, DudeState};

#[derive(Component, Default)]
pub struct EnemyStateMachine {
    just_changed: bool,
    state: DudeState,
    previous_state: DudeState,
    attack: Attack,
}

impl EnemyStateMachine {
    pub fn previous_state(&self) -> DudeState {
        self.previous_state
    }

    pub fn state(&self) -> DudeState {
        self.state
    }

    pub fn set_state(&mut self, state: DudeState) {
        if self.just_changed {
            let msg = format!(
                "
Trying to set state in ENEMY even though it was already changed this frame.
Should never happen, you probably forgot a flag check.
Current state: {:?}
Attempted new state: {:?}",
                self.state(),
                state
            );
            error!(msg);
            return;
        }

        self.set_just_changed(true);
        self.previous_state = self.state;
        self.state = state;
    }

    pub fn attack(&self) -> Attack {
        self.attack
    }

    pub fn attack_eq(&self, attack: Attack) -> bool {
        self.state == DudeState::Attacking && self.attack() == attack
    }

    /// Set the attack and also the state to attacking.
    pub fn set_attack(&mut self, attack: Attack) {
        if self.just_changed {
            error!("Trying to set state even though it was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }
        self.set_state(DudeState::Attacking);
        self.attack = attack;
    }

    pub fn just_changed(&self) -> bool {
        self.just_changed
    }

    pub fn set_just_changed(&mut self, just_changed: bool) {
        self.just_changed = just_changed;
    }
}
