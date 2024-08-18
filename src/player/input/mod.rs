mod controller;
mod relay;

pub use controller::PlayerInput;

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((controller::InputControllerPlugin, relay::InputRelayPlugin));
    }
}
