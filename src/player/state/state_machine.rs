use std::time::Duration;

use bevy::prelude::*;

use crate::dude::{Attack, AttackForm, DudeAnimations, DudeState, ParryState};

use super::AttackHandler;

#[derive(Component, Default)]
pub struct PlayerStateMachine {
    just_changed: bool,
    state: DudeState,
    previous_state: DudeState,
    parry_state: ParryState,
    new_state: Option<DudeState>,
    attack_handler: AttackHandler,
    animation_state: DudeAnimations,
}

impl PlayerStateMachine {
    pub fn can_run(&self) -> bool {
        self.state == DudeState::Idling
            || self.state == DudeState::Running
            || self.state == DudeState::Recovering
            || (self.state == DudeState::Parrying
                && (self.parry_state == ParryState::Success
                    || self.parry_state == ParryState::Recover))
    }

    fn can_attack(&self) -> bool {
        self.can_run() || self.state == DudeState::Attacking
    }

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
Trying to set state in PLAYER even though it was already changed this frame.
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

    pub fn new_state(&self) -> Option<DudeState> {
        self.new_state
    }

    pub fn set_new_state(&mut self, new_state: DudeState) {
        if self.new_state.is_some() {
            return;
        }
        self.new_state = Some(new_state);
    }

    pub fn reset_new_state(&mut self) {
        self.new_state = None;
    }

    pub fn parry_state(&self) -> ParryState {
        self.parry_state
    }

    pub fn set_parry_state(&mut self, parry_state: ParryState) {
        if self.just_changed {
            error!("Trying to set parry state even though state was already changed this frame. Should never happen, you probably forgot a flag check, current: {:?}, new: {:?}", self.state, parry_state);
            return;
        }
        self.set_state(DudeState::Parrying);
        self.parry_state = parry_state;
    }

    pub fn animation_state(&self) -> DudeAnimations {
        self.animation_state
    }

    pub fn set_animation_state(&mut self, animation_state: DudeAnimations) {
        self.animation_state = animation_state;
    }

    pub fn attack(&self) -> Attack {
        self.attack_handler.attack()
    }

    pub fn attack_eq(&self, attack: Attack) -> bool {
        self.state == DudeState::Attacking && self.attack() == attack
    }

    /// Set the attack and also the state to attacking.
    pub fn set_attack(&mut self, attack: Attack) {
        if self.just_changed {
            error!("Trying to set attack state even though state was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }
        self.set_state(DudeState::Attacking);
        self.attack_handler.set_attack(attack);
        self.attack_handler.set_chainable(true);
    }

    pub fn attack_direction(&self) -> Vec2 {
        self.attack_handler.attack_direction()
    }

    pub fn set_attack_direction(&mut self, direction: Vec2) {
        self.attack_handler.set_attack_direction(direction);
    }

    pub fn chained_attack(&self) -> AttackForm {
        self.attack_handler.chained_attack()
    }

    pub fn set_chained_attack(&mut self, chained_attack: AttackForm) {
        self.attack_handler.set_chained_attack(chained_attack);
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

    pub fn default_attack(&mut self, attack_form: AttackForm) {
        match attack_form {
            AttackForm::None => {}
            AttackForm::Light => self.set_attack(Attack::Light1),
            AttackForm::Heavy => self.set_attack(Attack::Heavy1),
        }
    }

    pub fn combo_attack(&self, attack_form: AttackForm) -> Option<Attack> {
        match self.attack() {
            Attack::Light1 => match attack_form {
                AttackForm::None => panic!("should never happen!"),
                AttackForm::Light => Some(Attack::Light2),
                AttackForm::Heavy => Some(Attack::Heavy1),
            },
            Attack::Light2 => match attack_form {
                AttackForm::None => panic!("should never happen!"),
                AttackForm::Light => Some(Attack::Light3),
                AttackForm::Heavy => Some(Attack::Heavy3),
            },
            Attack::Light3 => match attack_form {
                AttackForm::None => panic!("should never happen!"),
                AttackForm::Light => None,
                AttackForm::Heavy => Some(Attack::Heavy2),
            },
            Attack::Heavy1 => match attack_form {
                AttackForm::None => panic!("should never happen!"),
                AttackForm::Light => Some(Attack::Light2),
                AttackForm::Heavy => Some(Attack::Heavy2),
            },
            Attack::Heavy2 => match attack_form {
                AttackForm::None => panic!("should never happen!"),
                AttackForm::Light => None,
                AttackForm::Heavy => Some(Attack::Heavy3),
            },
            Attack::Heavy3 => None,
        }
    }

    pub fn transition_chain_attack(&mut self, move_direction: Vec2) {
        let terminal_state = if move_direction == Vec2::ZERO {
            DudeState::Recovering
        } else {
            DudeState::Running
        };
        if self.chained_attack() == AttackForm::None {
            self.set_state(terminal_state);
            return;
        }

        match self.combo_attack(self.chained_attack()) {
            Some(attack) => self.set_attack(attack),
            None => self.set_state(terminal_state),
        }
        self.set_chained_attack(AttackForm::None);
    }

    pub fn transition_attack(&mut self, attack_form: AttackForm) {
        if self.just_changed() {
            return;
        }
        if !self.can_attack() {
            return;
        }

        if self.state() == DudeState::Attacking {
            assert_ne!(attack_form, AttackForm::None);
            self.set_chained_attack(attack_form);
        } else if self.attack_handler.chainable() {
            match self.combo_attack(attack_form) {
                Some(attack) => self.set_attack(attack),
                None => self.default_attack(attack_form),
            }
        } else {
            self.default_attack(attack_form);
        }
    }
}
