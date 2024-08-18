use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use bevy_rancic::prelude::*;

use crate::GameState;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
    pub scroll: i32,
    pub escape: bool,
    pub punched: bool,

    pub mouse_world_coords: Vec2,

    pub toggle_fullscreen: bool,
    pub toggle_debug: bool,
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}

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

fn input_scroll(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut zoom = 0;
    if keys.just_pressed(KeyCode::Backspace) {
        zoom -= 1;
    }
    if keys.just_pressed(KeyCode::Minus) {
        zoom += 1;
    }
    player_input.scroll = zoom;
}

fn player_movement(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut direction = Vec2::default();

    if keys.pressed(KeyCode::KeyJ) || keys.pressed(KeyCode::KeyS) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keys.pressed(KeyCode::KeyK) || keys.pressed(KeyCode::KeyW) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keys.pressed(KeyCode::KeyF) || keys.pressed(KeyCode::KeyD) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyA) {
        direction += Vec2::new(-1.0, 0.0);
    }

    player_input.move_direction = direction.normalize_or_zero();
}

fn input_escape(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.escape = keys.just_pressed(KeyCode::Escape);
}

fn input_punched(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut player_input: ResMut<PlayerInput>,
) {
    player_input.punched =
        keys.just_pressed(KeyCode::KeyL) || mouse_buttons.just_pressed(MouseButton::Left);
}

fn toggle_fullscreen(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.toggle_fullscreen = keys.just_pressed(KeyCode::KeyB);
}

fn toggle_debug(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.toggle_debug = keys.just_pressed(KeyCode::F3);
}

pub struct InputControllerPlugin;

impl Plugin for InputControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                fetch_scroll_events,
                fetch_mouse_world_coords,
                input_scroll,
                player_movement,
                input_escape,
                input_punched,
                toggle_fullscreen,
                toggle_debug,
            )
                .run_if(in_state(GameState::Gaming))
                .after(InputSystem),
        )
        .init_resource::<PlayerInput>()
        .add_systems(PreUpdate, reset_player_input.before(InputSystem));
    }
}
