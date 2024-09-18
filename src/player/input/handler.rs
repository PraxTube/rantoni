use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use bevy_rancic::prelude::*;

use crate::player::Player;
use crate::GameState;

use super::gamepad::PlayerGamepad;
use super::PlayerInput;

pub fn fetch_mouse_world_coords(
    mut player_input: ResMut<PlayerInput>,
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
        player_input.mouse_world_coords = world_position;
    }
}

fn fetch_scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut player_input: ResMut<PlayerInput>,
) {
    for ev in scroll_evr.read() {
        let scroll = match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0.0 {
                    -1
                } else {
                    1
                }
            }
            MouseScrollUnit::Pixel => {
                if ev.y > 0.0 {
                    -1
                } else {
                    1
                }
            }
        };
        player_input.scroll = scroll;
    }
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    axes: Res<Axis<GamepadAxis>>,
    player_gamepad: Res<PlayerGamepad>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut keyboard_direction = Vec2::default();

    if keys.pressed(KeyCode::KeyJ) || keys.pressed(KeyCode::KeyS) {
        keyboard_direction += Vec2::new(0.0, -1.0);
    }
    if keys.pressed(KeyCode::KeyK) || keys.pressed(KeyCode::KeyW) {
        keyboard_direction += Vec2::new(0.0, 1.0);
    }
    if keys.pressed(KeyCode::KeyF) || keys.pressed(KeyCode::KeyD) {
        keyboard_direction += Vec2::new(1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyA) {
        keyboard_direction += Vec2::new(-1.0, 0.0);
    }

    let gampepad_direction = if let Some(gamepad) = player_gamepad.gamepad {
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
        Vec2::new(x, y)
    } else {
        Vec2::ZERO
    };

    let dir = if keyboard_direction != Vec2::ZERO {
        keyboard_direction
    } else {
        gampepad_direction
    };

    player_input.move_direction = dir.normalize_or_zero();
}

fn player_aim_direction(
    axes: Res<Axis<GamepadAxis>>,
    player_gamepad: Res<PlayerGamepad>,
    mut player_input: ResMut<PlayerInput>,
    q_player: Query<&Transform, With<Player>>,
) {
    let gampepad_direction = if let Some(gamepad) = player_gamepad.gamepad {
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
        Vec2::new(x, y)
    } else {
        Vec2::ZERO
    };

    let dir = if gampepad_direction != Vec2::ZERO {
        gampepad_direction
    } else {
        match q_player.get_single() {
            Ok(transform) => player_input.mouse_world_coords - transform.translation.truncate(),
            Err(_) => Vec2::ZERO,
        }
    };

    player_input.aim_direction = dir.normalize_or_zero();
}

fn handle_keyboard_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut input = PlayerInput {
        escape: keys.just_pressed(KeyCode::Escape),
        toggle_fullscreen: keys.just_pressed(KeyCode::KeyB),
        toggle_debug: keys.just_pressed(KeyCode::F3),
        ..default()
    };

    input.light_attack =
        keys.just_pressed(KeyCode::KeyL) || mouse_buttons.just_pressed(MouseButton::Left);
    input.heavy_attack =
        keys.just_pressed(KeyCode::KeyN) || mouse_buttons.just_pressed(MouseButton::Right);

    let mut zoom = 0;
    if keys.just_pressed(KeyCode::Backspace) {
        zoom -= 1;
    }
    if keys.just_pressed(KeyCode::Minus) {
        zoom += 1;
    }
    input.scroll = zoom;

    *player_input |= input;
}

fn handle_gamepad_inputs(
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    player_gamepad: Res<PlayerGamepad>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut input = PlayerInput::default();
    let Some(gamepad) = player_gamepad.gamepad else {
        return;
    };

    input.light_attack =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South));
    input.heavy_attack =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::West));
    input.toggle_fullscreen =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown));
    input.toggle_debug =
        gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp));

    let mut zoom = 0;
    if gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)) {
        zoom -= 1;
    }
    if gamepad_buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)) {
        zoom += 1;
    }
    input.scroll = zoom;

    *player_input |= input;
}

pub struct InputControllerPlugin;

impl Plugin for InputControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                fetch_mouse_world_coords,
                fetch_scroll_events,
                player_movement,
                player_aim_direction,
                handle_keyboard_inputs,
                handle_gamepad_inputs,
            )
                .chain()
                .run_if(in_state(GameState::Gaming))
                .after(InputSystem),
        );
    }
}
