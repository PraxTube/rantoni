use bevy::prelude::*;

use super::Attack;

#[derive(Default, Clone, Copy)]
pub enum StaggerState {
    #[default]
    Normal,
    StanceBreak,
}

#[derive(Component, Default)]
pub struct Stagger {
    pub state: StaggerState,
    pub direction: Vec2,
    pub intensity: f32,
    timer: Timer,
    use_animation_end: bool,
}

impl Stagger {
    fn new_state(&mut self, state: StaggerState, direction: Vec2, duration: f32, intensity: f32) {
        self.state = state;
        self.direction = direction;
        self.intensity = intensity;

        if duration != 0.0 {
            self.timer = Timer::from_seconds(duration, TimerMode::Once);
            self.use_animation_end = false;
        } else {
            self.timer.reset();
            self.use_animation_end = true;
        }
    }

    pub fn update(
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
                    50.0 * intensity_multiplier,
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
                    0.2 * duration_multiplier,
                    0.0 * intensity_multiplier,
                );
            }
            Attack::Heavy1 => {
                self.new_state(
                    StaggerState::Normal,
                    direction,
                    0.3 * duration_multiplier,
                    0.0 * intensity_multiplier,
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
                    StaggerState::Normal,
                    direction,
                    0.25 * duration_multiplier,
                    700.0 * intensity_multiplier,
                );
            }
        }
    }

    pub fn set_stance_break(&mut self) {
        self.new_state(StaggerState::StanceBreak, Vec2::ZERO, 0.0, 0.0);
    }

    /// The player should always get the exact same amount of knockback regardless of the actual
    /// attack. So always use this for player stagger.
    pub fn set_player_stagger(&mut self, direction: Vec2) {
        self.new_state(StaggerState::Normal, direction, 0.3, 150.0);
    }

    pub fn just_finished(&self) -> bool {
        self.timer.just_finished()
    }

    pub fn use_animation_end(&self) -> bool {
        self.use_animation_end
    }
}

fn tick_stagger_timers(time: Res<Time>, mut q_staggers: Query<&mut Stagger>) {
    for mut stagger in &mut q_staggers {
        if stagger.use_animation_end {
            continue;
        }
        stagger.timer.tick(time.delta());
    }
}

pub struct StaggerPlugin;

impl Plugin for StaggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (tick_stagger_timers,));
    }
}
