use std::time::Duration;

use bevy::prelude::*;

use crate::dude::{
    Attack, AttackForm, DudeAnimations, DudeState, JumpingState, ParryState, Stagger, StaggerState,
};

use super::{jumping::Jumping, AttackHandler};

#[derive(Component, Default)]
pub struct PlayerStateMachine {
    just_changed: bool,
    state: DudeState,
    previous_state: DudeState,
    stagger: Stagger,
    jumping: Jumping,
    new_state: Option<DudeState>,
    attack_handler: AttackHandler,
    animation_state: DudeAnimations,
}

impl PlayerStateMachine {
    pub fn can_run(&self) -> bool {
        self.state == DudeState::Idling
            || self.state == DudeState::Running
            || (self.state == DudeState::Recovering
                && (self.attack() != Attack::Dropkick && self.attack() != Attack::Hammerfist))
            || self.state == DudeState::Parrying(ParryState::Success)
            || self.state == DudeState::Parrying(ParryState::Recover)
            || self.state == DudeState::Staggering && self.stagger.state().is_recovering()
    }

    pub fn can_attack(&self) -> bool {
        self.can_run()
            || (self.state == DudeState::Attacking
                && self.attack() != Attack::Slide
                && self.attack() != Attack::Dropkick
                && self.attack() != Attack::Hammerfist)
    }

    pub fn previous_state(&self) -> DudeState {
        self.previous_state
    }

    pub fn state(&self) -> DudeState {
        self.state
    }

    pub fn set_state(&mut self, state: DudeState) {
        // TODO: Figure this out, is it necessary?
        // if state == self.state {
        //     warn!(
        //         "Setting new state to old state (already target state), state: {:?}",
        //         state
        //     );
        // }
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
        self.attack_handler.set_can_move(true);
    }

    pub fn new_state(&self) -> Option<DudeState> {
        self.new_state
    }

    pub fn set_new_state(&mut self, new_state: DudeState) {
        // TODO: Should I handle this? Should this never happen? asserts, or just warning? Same
        // with enemy statemachine.
        if self.new_state.is_some() {
            return;
        }
        self.new_state = Some(new_state);
    }

    pub fn reset_new_state(&mut self) {
        self.new_state = None;
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

    pub fn set_sliding_attack(&mut self) {
        if self.just_changed {
            error!("Trying to set sliding state even though state was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }
        self.set_state(DudeState::Attacking);
        self.attack_handler.set_attack(Attack::Slide);
        self.attack_handler.set_chainable(false);
        self.attack_handler.set_chained_attack(AttackForm::None);
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

    pub fn set_default_attack(&mut self, attack_form: AttackForm) {
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
            Attack::Slide => None,
            Attack::Dropkick => None,
            Attack::Hammerfist => None,
        }
    }

    pub fn transition_chain_attack(&mut self, move_direction: Vec2) {
        let terminal_state = if move_direction == Vec2::ZERO
            || self.attack() == Attack::Dropkick
            || self.attack() == Attack::Hammerfist
        {
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
                None => self.set_default_attack(attack_form),
            }
        } else {
            self.set_default_attack(attack_form);
        }
    }

    pub fn tick_stagger_timer(&mut self, delta: Duration) {
        self.stagger.tick_timer(delta);
    }

    pub fn stagger_just_finished(&self) -> bool {
        self.stagger.just_finished()
    }

    pub fn stagger_linvel(&self) -> Vec2 {
        self.stagger.linvel()
    }

    pub fn stagger_state(&self) -> StaggerState {
        self.stagger.state()
    }

    pub fn set_stagger_state(&mut self, direction: Vec2) {
        self.set_new_state(DudeState::Staggering);
        self.stagger
            .new_state(StaggerState::Normal, direction, 0.3, 150.0);
    }

    pub fn set_stagger_state_recover(&mut self) {
        self.stagger.set_recover_state();
    }

    pub fn sprite_y_offset(&self) -> f32 {
        self.jumping.sprite_y_offset()
    }

    pub fn tick_jumping_timer(&mut self, delta: Duration) {
        self.jumping.tick_timer(delta);
    }

    pub fn jumping_timer_finished(&self) -> bool {
        self.jumping.finished()
    }

    pub fn jumping_duration(&self) -> f32 {
        self.jumping.duration()
    }

    pub fn set_jumping_duration(&mut self, duration: f32) {
        self.jumping.set_duration(duration);
    }

    pub fn reset_jumping_timer(&mut self) {
        self.jumping.reset_timer();
    }

    pub fn jumping_linvel(&self, direction: Vec2) -> Vec2 {
        match self.state {
            DudeState::Jumping(jumping_state) => self.jumping.linvel(jumping_state, direction),
            DudeState::Attacking => self.jumping.linvel(JumpingState::Start, direction),
            _ => {
                error!("should never happen");
                Vec2::ZERO
            }
        }
    }

    /// This will be `true` at the start of the animation and will turn `false` once the hitbox
    /// gets activated.
    pub fn can_move_during_attack(&self) -> bool {
        self.attack_handler.can_move()
    }

    pub fn disable_can_move_during_attack(&mut self) {
        self.attack_handler.set_can_move(false);
    }
}
