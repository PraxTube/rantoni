use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

use crate::player::Player;
use crate::world::MainCamera;
use crate::GameState;

use super::gamepad::PlayerGamepad;
use super::{GamingInput, InputControllerSystem, InputDevice};

fn fetch_mouse_world_coords(
    mut gaming_input: ResMut<GamingInput>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = match q_camera.get_single() {
        Ok(c) => (c.0, c.1),
        Err(_) => return,
    };
    let window = match q_window.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        gaming_input.mouse_world_coords = world_position;
    }
}

fn update_aim_direction(
    mut gaming_input: ResMut<GamingInput>,
    q_players: Query<&Transform, With<Player>>,
    input_device: Res<InputDevice>,
) {
    if *input_device != InputDevice::MouseKeyboard {
        return;
    }

    for transform in &q_players {
        let dir = gaming_input.mouse_world_coords - transform.translation.truncate();

        if dir != Vec2::ZERO {
            gaming_input.aim_direction = dir.normalize_or_zero();
        }
    }
}

fn handle_keyboard_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut gaming_input: ResMut<GamingInput>,
    mut input_device: ResMut<InputDevice>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let mut input = GamingInput::default();

    input.light_attack =
        keys.just_pressed(KeyCode::KeyL) || mouse_buttons.just_pressed(MouseButton::Left);
    input.heavy_attack =
        keys.just_pressed(KeyCode::KeyN) || mouse_buttons.just_pressed(MouseButton::Right);
    input.parry = keys.just_pressed(KeyCode::KeyP) || keys.just_pressed(KeyCode::KeyE);
    input.dash = keys.just_pressed(KeyCode::ShiftLeft) || keys.just_pressed(KeyCode::KeyZ);
    input.special_light = keys.just_pressed(KeyCode::KeyQ);
    input.special_heavy = keys.just_pressed(KeyCode::Space);

    let mut move_direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyJ) || keys.pressed(KeyCode::KeyS) {
        move_direction += Vec2::NEG_Y;
    }
    if keys.pressed(KeyCode::KeyK) || keys.pressed(KeyCode::KeyW) {
        move_direction += Vec2::Y;
    }
    if keys.pressed(KeyCode::KeyF) || keys.pressed(KeyCode::KeyD) {
        move_direction += Vec2::X;
    }
    if keys.pressed(KeyCode::KeyA) {
        move_direction += Vec2::NEG_X;
    }
    input.move_direction = move_direction.normalize_or_zero();

    let mut zoom = 0;
    if keys.just_pressed(KeyCode::Backspace) {
        zoom -= 1;
    }
    if keys.just_pressed(KeyCode::Minus) {
        zoom += 1;
    }

    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0.0 {
                    zoom -= 1;
                } else {
                    zoom += 1;
                }
            }
            MouseScrollUnit::Pixel => {
                if ev.y > 0.0 {
                    zoom -= 1;
                } else {
                    zoom += 1;
                }
            }
        };
    }
    input.scroll = zoom;

    if input != GamingInput::default() {
        *input_device = InputDevice::MouseKeyboard;
    }
    *gaming_input |= input;
}

fn handle_gamepad_inputs(
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    player_gamepad: Res<PlayerGamepad>,
    mut gaming_input: ResMut<GamingInput>,
    mut input_device: ResMut<InputDevice>,
) {
    let mut input = GamingInput::default();
    let Some(gamepad) = player_gamepad.gamepad else {
        return;
    };

    input.light_attack =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::West));
    input.heavy_attack =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::North));
    input.parry =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger));
    input.dash =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2));
    input.special_light =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::East));
    input.special_heavy =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South));

    let mut zoom = 0;
    if gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)) {
        zoom -= 1;
    }
    if gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)) {
        zoom += 1;
    }
    input.scroll = zoom;

    let left_stick_direction = {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        let (x, y) = (
            axes.get(axis_lx).unwrap_or_default(),
            axes.get(axis_ly).unwrap_or_default(),
        );
        Vec2::new(x, y).normalize_or_zero()
    };
    input.move_direction = left_stick_direction;
    input.aim_direction = left_stick_direction;

    if input != GamingInput::default() {
        *input_device = InputDevice::Gamepad;
    }
    *gaming_input |= input;
}

pub struct GamingInputPlugin;

impl Plugin for GamingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                fetch_mouse_world_coords,
                update_aim_direction,
                handle_keyboard_inputs,
                handle_gamepad_inputs,
            )
                .chain()
                .run_if(in_state(GameState::Gaming))
                .in_set(InputControllerSystem)
                .after(InputSystem),
        );
    }
}
