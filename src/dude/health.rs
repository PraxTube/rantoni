use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self { health }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems();
    }
}
