mod auto_input;
mod gamepad;
#[cfg(not(feature = "auto_input"))]
mod handler;
mod relay;

use std::ops::BitOrAssign;

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
        .insert_resource(InputDevice::MouseKeyboard)
        .add_systems(PreUpdate, reset_player_input.before(InputSystem));
    }
}

#[derive(Resource, PartialEq)]
enum InputDevice {
    MouseKeyboard,
    Gamepad,
}

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct PlayerInput {
    pub scroll: i32,
    pub escape: bool,

    pub move_direction: Vec2,
    pub light_attack: bool,
    pub heavy_attack: bool,
    pub parry: bool,

    pub mouse_world_coords: Vec2,

    pub toggle_fullscreen: bool,
    pub toggle_debug: bool,
}

impl BitOrAssign for PlayerInput {
    fn bitor_assign(&mut self, rhs: Self) {
        if self.move_direction == Vec2::ZERO {
            self.move_direction = rhs.move_direction;
        }
        if self.mouse_world_coords == Vec2::ZERO {
            self.mouse_world_coords = rhs.mouse_world_coords;
        }
        if self.scroll == 0 {
            self.scroll = rhs.scroll;
        }

        self.light_attack |= rhs.light_attack;
        self.heavy_attack |= rhs.heavy_attack;
        self.parry |= rhs.parry;
        self.escape |= rhs.escape;
        self.toggle_debug |= rhs.toggle_debug;
        self.toggle_fullscreen |= rhs.toggle_fullscreen;
    }
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}
