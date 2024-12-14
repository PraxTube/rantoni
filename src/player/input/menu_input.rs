use bevy::input::InputSystem;
use bevy::prelude::*;

use crate::GameState;

use super::gamepad::PlayerGamepad;
use super::{InputDevice, MenuInput};

fn handle_keyboard_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut gaming_input: ResMut<MenuInput>,
    mut input_device: ResMut<InputDevice>,
) {
    let mut input = MenuInput::default();

    input.restart = keys.just_pressed(KeyCode::KeyR);

    if input != MenuInput::default() {
        *input_device = InputDevice::MouseKeyboard;
    }
    *gaming_input |= input;
}

fn handle_gamepad_inputs(
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    player_gamepad: Res<PlayerGamepad>,
    mut gaming_input: ResMut<MenuInput>,
    mut input_device: ResMut<InputDevice>,
) {
    let mut input = MenuInput::default();
    let Some(gamepad) = player_gamepad.gamepad else {
        return;
    };

    input.restart =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South));

    if input != MenuInput::default() {
        *input_device = InputDevice::Gamepad;
    }
    *gaming_input |= input;
}

pub struct MenuInputPlugin;

impl Plugin for MenuInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (handle_keyboard_inputs, handle_gamepad_inputs)
                .chain()
                .run_if(in_state(GameState::GameOver))
                .after(InputSystem),
        );
    }
}
