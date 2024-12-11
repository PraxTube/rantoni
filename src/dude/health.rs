use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: u32,
}

impl Health {
    pub fn new(health: u32) -> Self {
        Self { health }
    }

    pub fn reduce(&mut self, amount: u32) {
        if amount > self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems();
    }
}
