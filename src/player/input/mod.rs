mod gamepad;
#[cfg(not(feature = "auto_input"))]
mod handler;
// #[cfg(feature = "auto_input")]
mod auto_input;
mod relay;

use bevy::{input::InputSystem, prelude::*};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            #[cfg(feature = "auto_input")]
            auto_input::InputTestingPlugin,
            #[cfg(not(feature = "auto_input"))]
            handler::InputControllerPlugin,
            relay::InputRelayPlugin,
            gamepad::InputGamepadPlugin,
        ))
        .init_resource::<PlayerInput>()
        .add_systems(PreUpdate, reset_player_input.before(InputSystem));
    }
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub scroll: i32,
    pub escape: bool,

    pub move_direction: Vec2,
    pub punched: bool,
    pub aim_direction: Vec2,

    pub mouse_world_coords: Vec2,

    pub toggle_fullscreen: bool,
    pub toggle_debug: bool,
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}
