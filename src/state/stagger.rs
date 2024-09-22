use bevy::prelude::*;

use super::Attack;

#[derive(Default)]
pub enum StaggerState {
    #[default]
    Normal,
    Flying,
}

#[derive(Default)]
pub struct Stagger {
    pub state: StaggerState,
    pub direction: Vec2,
    pub timer: Timer,
    pub intensity: f32,
}

impl Stagger {
    fn reset_state(&mut self, state: StaggerState, direction: Vec2, duration: f32, intensity: f32) {
        self.state = state;
        self.direction = direction;
        self.timer = Timer::from_seconds(duration, TimerMode::Once);
        self.intensity = intensity;
    }

    pub fn on_attack(
        &mut self,
        attack: Attack,
        direction: Vec2,
        duration_multiplier: f32,
        intensity_multiplier: f32,
    ) {
        match attack {
            Attack::Light1 => {
                self.reset_state(
                    StaggerState::Normal,
                    direction,
                    0.3 * duration_multiplier,
                    50.0 * intensity_multiplier,
                );
            }
            Attack::Light2 => {
                self.reset_state(
                    StaggerState::Normal,
                    direction,
                    0.3 * duration_multiplier,
                    250.0 * intensity_multiplier,
                );
            }
            Attack::Light3 => {
                self.reset_state(
                    StaggerState::Flying,
                    direction,
                    0.2 * duration_multiplier,
                    0.0 * intensity_multiplier,
                );
            }
            Attack::Heavy1 => {
                self.reset_state(
                    StaggerState::Normal,
                    direction,
                    0.3 * duration_multiplier,
                    0.0 * intensity_multiplier,
                );
            }
            Attack::Heavy2 => {
                self.reset_state(
                    StaggerState::Normal,
                    direction,
                    0.35 * duration_multiplier,
                    500.0 * intensity_multiplier,
                );
            }
            Attack::Heavy3 => {
                self.reset_state(
                    StaggerState::Normal,
                    direction,
                    0.25 * duration_multiplier,
                    700.0 * intensity_multiplier,
                );
            }
        }
    }
}
