use std::time::Duration;

use bevy::prelude::*;

use crate::dude::{Attack, DudeState, EnemyAnimations, Stagger, StaggerState};

use super::attack::AttackHandler;

#[derive(Component, Default)]
pub struct EnemyStateMachine {
    just_changed: bool,
    state: DudeState,
    previous_state: DudeState,
    stagger: Stagger,
    new_state: Option<DudeState>,
    animation_state: EnemyAnimations,
    attack_handler: AttackHandler,
}

impl EnemyStateMachine {
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

    pub fn can_attack(&self) -> bool {
        self.state() == DudeState::Idling || self.state() == DudeState::Running
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

    pub fn animation_state(&self) -> EnemyAnimations {
        self.animation_state
    }

    pub fn set_animation_state(&mut self, animation_state: EnemyAnimations) {
        self.animation_state = animation_state;
    }

    pub fn attack(&self) -> Attack {
        self.attack_handler.attack()
    }

    #[allow(dead_code)]
    pub fn attack_eq(&self, attack: Attack) -> bool {
        self.state == DudeState::Attacking && self.attack() == attack
    }

    /// Set the attack and also the state to attacking.
    pub fn set_attack(&mut self, attack: Attack, attack_direction: Vec2) {
        if self.just_changed {
            error!("Trying to set state even though it was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }
        self.set_state(DudeState::Attacking);
        self.attack_handler.set_attack(attack, attack_direction);
    }

    pub fn attack_timer_finished(&self) -> bool {
        self.attack_handler.attack_timer_finished()
    }

    pub fn tick_attack_timer(&mut self, delta: Duration) {
        self.attack_handler.tick_attack_timer(delta);
    }

    pub fn reset_attack_timer(&mut self) {
        self.attack_handler.reset_attack_timer();
    }

    pub fn attack_direction(&self) -> Vec2 {
        self.attack_handler.attack_direction()
    }

    pub fn just_changed(&self) -> bool {
        self.just_changed
    }

    pub fn set_just_changed(&mut self, just_changed: bool) {
        self.just_changed = just_changed;
    }

    pub fn tick_stagger_timer(&mut self, delta: Duration) {
        self.stagger.tick_timer(delta);
    }

    pub fn stagger_finished(&self) -> bool {
        self.stagger.just_finished()
    }

    pub fn stagger_linvel(&self) -> Vec2 {
        self.stagger.linvel()
    }

    pub fn set_stagger_state(
        &mut self,
        attack: Attack,
        direction: Vec2,
        duration_multiplier: f32,
        intensity_multiplier: f32,
    ) {
        self.set_new_state(DudeState::Staggering);
        self.stagger.stagger_from_attack(
            attack,
            direction,
            duration_multiplier,
            intensity_multiplier,
        );
    }

    pub fn set_stagger_stance_break_state(&mut self) {
        self.set_new_state(DudeState::Staggering);
        self.stagger
            .new_state(StaggerState::StanceBreak, Vec2::ZERO, 0.5, 0.0);
    }

    pub fn stagger_state(&self) -> StaggerState {
        self.stagger.state()
    }

    pub fn set_stagger_state_recover(&mut self) {
        self.stagger.set_recover_state();
    }
}
