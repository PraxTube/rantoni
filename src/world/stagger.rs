use bevy::prelude::*;

#[derive(Default)]
pub struct Stagger {
    pub direction: Vec2,
    pub timer: Timer,
    pub intensity: f32,
}

impl Stagger {
    pub fn new(direction: Vec2, duration: f32, intensity: f32) -> Self {
        Self {
            direction,
            timer: Timer::from_seconds(duration, TimerMode::Once),
            intensity,
        }
    }
}
