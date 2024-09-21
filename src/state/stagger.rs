use bevy::prelude::*;

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
    pub fn new(state: StaggerState, direction: Vec2, duration: f32, intensity: f32) -> Self {
        Self {
            state,
            direction,
            timer: Timer::from_seconds(duration, TimerMode::Once),
            intensity,
        }
    }
}
