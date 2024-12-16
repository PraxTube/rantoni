use bevy::input::InputSystem;
use bevy::prelude::*;

use super::gamepad::PlayerGamepad;
use super::GlobalInput;

fn handle_keyboard_inputs(keys: Res<ButtonInput<KeyCode>>, mut global_input: ResMut<GlobalInput>) {
    let mut input = GlobalInput::default();

    input.toggle_fullscreen = keys.just_pressed(KeyCode::KeyB);
    input.toggle_debug = keys.just_pressed(KeyCode::F3);
    input.toggle_grid_debug_visuals = keys.just_pressed(KeyCode::KeyG);

    *global_input |= input;
}

fn handle_gamepad_inputs(
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    player_gamepad: Res<PlayerGamepad>,
    mut global_input: ResMut<GlobalInput>,
) {
    let mut input = GlobalInput::default();
    let Some(gamepad) = player_gamepad.gamepad else {
        return;
    };

    input.toggle_fullscreen =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown));
    input.toggle_debug =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp));
    input.toggle_grid_debug_visuals =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft));

    *global_input |= input;
}

pub struct GlobalInputPlugin;

impl Plugin for GlobalInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (handle_keyboard_inputs, handle_gamepad_inputs)
                .chain()
                .after(InputSystem),
        );
    }
}
