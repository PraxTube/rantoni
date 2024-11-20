use std::time::Duration;

use bevy::prelude::*;

use crate::{enemy::Enemy, player::Player};

use super::Attack;

#[derive(Default, Clone, Copy)]
pub enum StaggerState {
    #[default]
    Normal,
    StanceBreak,
    Fall,
    NormalRecover,
    StanceBreakRecover,
    FallRecover,
}

#[derive(Default)]
pub struct Stagger {
    state: StaggerState,
    direction: Vec2,
    intensity: f32,
    timer: Timer,
}

impl StaggerState {
    pub fn is_recovering(&self) -> bool {
        match self {
            StaggerState::Normal => false,
            StaggerState::StanceBreak => false,
            StaggerState::Fall => false,
            StaggerState::NormalRecover => true,
            StaggerState::StanceBreakRecover => true,
            StaggerState::FallRecover => true,
        }
    }
}

impl Stagger {
    pub fn new_state(
        &mut self,
        state: StaggerState,
        direction: Vec2,
        duration: f32,
        intensity: f32,
    ) {
        assert!(duration != 0.0);
        self.state = state;
        self.direction = direction;
        self.intensity = intensity;
        self.timer = Timer::from_seconds(duration, TimerMode::Once);
    }

    pub fn stagger_from_attack(
        &mut self,
        attack: Attack,
        direction: Vec2,
        duration_multiplier: f32,
        intensity_multiplier: f32,
    ) {
        match attack {
            Attack::Light1 => {
                self.new_state(
                    StaggerState::Normal,
                    direction,
                    0.3 * duration_multiplier,
                    75.0 * intensity_multiplier,
                );
            }
            Attack::Light2 => {
                self.new_state(
                    StaggerState::Normal,
                    direction,
                    0.3 * duration_multiplier,
                    250.0 * intensity_multiplier,
                );
            }
            Attack::Light3 => {
                self.new_state(
                    StaggerState::Normal,
                    direction,
                    0.25 * duration_multiplier,
                    150.0 * intensity_multiplier,
                );
            }
            Attack::Heavy1 => {
                self.new_state(
                    StaggerState::Normal,
                    direction,
                    0.2 * duration_multiplier,
                    50.0 * intensity_multiplier,
                );
            }
            Attack::Heavy2 => {
                self.new_state(
                    StaggerState::Normal,
                    direction,
                    0.35 * duration_multiplier,
                    500.0 * intensity_multiplier,
                );
            }
            Attack::Heavy3 => {
                self.new_state(
                    StaggerState::Fall,
                    direction,
                    0.3 * duration_multiplier,
                    0.0 * intensity_multiplier,
                );
            }
            Attack::Dropkick => {
                self.new_state(
                    StaggerState::Normal,
                    direction,
                    0.3 * duration_multiplier,
                    1500.0 * intensity_multiplier,
                );
            }
            Attack::Hammerfist => {
                self.new_state(
                    StaggerState::Fall,
                    direction,
                    0.4 * duration_multiplier,
                    0.0 * intensity_multiplier,
                );
            }
        }
    }

    pub fn tick_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn just_finished(&self) -> bool {
        self.timer.just_finished()
    }

    pub fn linvel(&self) -> Vec2 {
        self.direction * self.intensity
    }

    pub fn state(&self) -> StaggerState {
        self.state
    }

    pub fn set_recover_state(&mut self) {
        let next_state = match self.state {
            StaggerState::Normal => StaggerState::NormalRecover,
            StaggerState::StanceBreak => StaggerState::StanceBreakRecover,
            StaggerState::Fall => StaggerState::FallRecover,
            _ => {
                error!("should never happen, trying to set recover state but state is already recover state");
                StaggerState::NormalRecover
            }
        };
        self.state = next_state;
    }
}

fn tick_player_stagger_timers(time: Res<Time>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        player.state_machine.tick_stagger_timer(time.delta());
    }
}

fn tick_enemy_stagger_timers(time: Res<Time>, mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        enemy.state_machine.tick_stagger_timer(time.delta());
    }
}

pub struct StaggerPlugin;

impl Plugin for StaggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (tick_player_stagger_timers, tick_enemy_stagger_timers),
        );
    }
}
