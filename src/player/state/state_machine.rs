use std::time::Duration;

use bevy::prelude::*;

use crate::dude::{
    Attack, AttackForm, DudeState, ParryState, PlayerAnimations, Stagger, StaggerState,
};

use super::{dashing::DashingTimer, jumping::Jumping, AttackHandler};

#[derive(Component, Default)]
pub struct PlayerStateMachine {
    just_changed: bool,
    state: DudeState,
    previous_state: DudeState,
    dashing_timer: DashingTimer,
    stagger: Stagger,
    jumping: Jumping,
    new_state: Option<DudeState>,
    attack_handler: AttackHandler,
    animation_state: PlayerAnimations,
}

impl PlayerStateMachine {
    pub fn can_run(&self) -> bool {
        self.state == DudeState::Idling
            || self.state == DudeState::Running
            || (self.state == DudeState::Recovering
                && (self.attack() != Attack::Dropkick && self.attack() != Attack::Hammerfist))
            || self.state == DudeState::Parrying(ParryState::Success)
            || self.state == DudeState::Staggering && self.stagger.state().is_recovering()
    }

    pub fn can_attack(&self) -> bool {
        self.can_run()
            || (self.state == DudeState::Attacking
                && self.attack() != Attack::Dropkick
                && self.attack() != Attack::Hammerfist)
    }

    pub fn can_change_direction(&self) -> bool {
        self.state != DudeState::Attacking
            && self.state != DudeState::Recovering
            && self.state != DudeState::Dying
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
        assert_eq!(new_state, DudeState::Staggering, "If you want to use this for other states, then you will need to handle that in the `state/mod.rs` file too, i.e. you need to make sure that the new_state is either the target state or continue (return) and do nothing, see how the `transition_stagger_state` handles this for more info.");
        if self.new_state.is_some() {
            return;
        }
        self.new_state = Some(new_state);
    }

    pub fn reset_new_state(&mut self) {
        self.new_state = None;
    }

    pub fn animation_state(&self) -> PlayerAnimations {
        self.animation_state
    }

    pub fn set_animation_state(&mut self, animation_state: PlayerAnimations) {
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
        if direction == Vec2::ZERO {
            error!(
                "Trying to set attack direciton of player to Vec2::ZERO, doesn't make any sense"
            );
            return;
        }
        if self.state() == DudeState::Attacking {
            self.attack_handler.set_cached_attack_direction(direction);
        } else {
            self.attack_handler.set_attack_direction(direction);
        }
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
        if let Some(attack) = attack_form.to_default_attack() {
            self.set_attack(attack);
        }
    }

    pub fn combo_attack(&self, attack_form: AttackForm) -> Option<Attack> {
        self.attack().to_combo_attack(attack_form)
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
            Some(attack) => {
                self.set_attack(attack);
                self.attack_handler.exhaust_cached_attack_direction();
            }
            None => self.set_state(terminal_state),
        }
        self.set_chained_attack(AttackForm::None);
    }

    pub fn transition_attack(&mut self, attack_form: AttackForm) {
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

    pub fn tick_dashing_timer(&mut self, delta: Duration) {
        self.dashing_timer.0.tick(delta);
    }

    pub fn tick_stagger_timer(&mut self, delta: Duration) {
        self.stagger.tick_timer(delta);
    }

    pub fn dashing_just_finished(&self) -> bool {
        self.dashing_timer.0.just_finished()
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

    pub fn jumping_duration(&self) -> f32 {
        self.jumping.duration()
    }

    pub fn set_jumping_duration(&mut self, duration: f32) {
        self.jumping.set_duration(duration);
    }

    pub fn reset_jumping_timer(&mut self) {
        self.jumping.reset_timer();
    }

    pub fn jump_attack_speed_multiplier(&self) -> f32 {
        match self.state {
            DudeState::Attacking => self.jumping.speed(),
            _ => {
                error!("should never happen");
                0.0
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
