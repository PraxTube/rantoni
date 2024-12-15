use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::dynamics::IntegrationParameters};

pub struct WorldPhysicsPlugin;

impl Plugin for WorldPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_physics);
    }
}

fn configure_physics(
    mut rapier_config: ResMut<RapierConfiguration>,
    mut rapier_context: ResMut<RapierContext>,
) {
    rapier_config.gravity = Vec2::ZERO;
    rapier_context.integration_parameters = IntegrationParameters {
        normalized_max_corrective_velocity: 1.0e10,
        contact_damping_ratio: 0.0001,
        normalized_allowed_linear_error: 0.00001,
        ..default()
    };
}
