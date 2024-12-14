mod gamepad;
mod handler;
mod relay;

use std::ops::BitOrAssign;

use bevy::{input::InputSystem, prelude::*};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            handler::InputControllerPlugin,
            relay::InputRelayPlugin,
            gamepad::InputGamepadPlugin,
        ))
        .init_resource::<GamingInput>()
        .insert_resource(InputDevice::MouseKeyboard)
        .add_systems(PreUpdate, reset_gaming_input.before(InputSystem));
    }
}

#[derive(Resource, PartialEq)]
enum InputDevice {
    MouseKeyboard,
    Gamepad,
}

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct GamingInput {
    pub scroll: i32,

    pub move_direction: Vec2,
    pub aim_direction: Vec2,
    pub light_attack: bool,
    pub heavy_attack: bool,
    pub parry: bool,
    pub dash: bool,
    pub special_light: bool,
    pub special_heavy: bool,

    pub mouse_world_coords: Vec2,

    pub toggle_fullscreen: bool,
    pub toggle_debug: bool,
}

impl BitOrAssign for GamingInput {
    fn bitor_assign(&mut self, rhs: Self) {
        if self.move_direction == Vec2::ZERO {
            self.move_direction = rhs.move_direction;
        }
        if self.aim_direction == Vec2::ZERO {
            self.aim_direction = rhs.aim_direction;
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
        self.dash |= rhs.dash;
        self.special_light |= rhs.special_light;
        self.special_heavy |= rhs.special_heavy;
        self.toggle_debug |= rhs.toggle_debug;
        self.toggle_fullscreen |= rhs.toggle_fullscreen;
    }
}

fn reset_gaming_input(mut gaming_input: ResMut<GamingInput>) {
    *gaming_input = GamingInput::default();
}
