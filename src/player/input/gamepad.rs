use bevy::{input::gamepad::*, prelude::*};

use crate::GameState;

#[derive(Resource, Default)]
pub struct PlayerGamepad {
    pub gamepad: Option<Gamepad>,
}

fn handle_gamepad_connection(
    mut player_gamepad: ResMut<PlayerGamepad>,
    mut ev_gamepad_connections: EventReader<GamepadConnectionEvent>,
) {
    for ev in ev_gamepad_connections.read() {
        match ev.connection {
            GamepadConnection::Connected(_) => {
                if player_gamepad.gamepad.is_none() {
                    player_gamepad.gamepad = Some(ev.gamepad);
                }
            }
            GamepadConnection::Disconnected => {
                if player_gamepad.gamepad == Some(ev.gamepad) {
                    player_gamepad.gamepad = None;
                }
            }
        }
    }
}

fn configure_gamepads(mut settings: ResMut<GamepadSettings>) {
    // add a larger default dead-zone to all axes (ignore small inputs, round to zero)
    settings.default_axis_settings.set_deadzone_lowerbound(-0.2);
    settings.default_axis_settings.set_deadzone_upperbound(0.2);

    // for buttons (or axes treated as buttons):
    let mut button_settings = ButtonSettings::default();
    // require them to be pressed almost all the way, to count
    button_settings.set_press_threshold(0.5);

    settings.default_button_settings = button_settings;
}

pub struct InputGamepadPlugin;

impl Plugin for InputGamepadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerGamepad>()
            .add_systems(OnExit(GameState::AssetLoading), configure_gamepads)
            .add_systems(Update, (handle_gamepad_connection,));
    }
}
