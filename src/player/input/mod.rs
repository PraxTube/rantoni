mod gamepad;
mod gaming_input;
mod global_input;
mod menu_input;

use std::ops::BitOrAssign;

use bevy::{input::InputSystem, prelude::*};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            global_input::GlobalInputPlugin,
            menu_input::MenuInputPlugin,
            gaming_input::GamingInputPlugin,
            gamepad::InputGamepadPlugin,
        ))
        .init_resource::<GlobalInput>()
        .init_resource::<MenuInput>()
        .init_resource::<GamingInput>()
        .insert_resource(InputDevice::MouseKeyboard)
        .add_systems(PreUpdate, reset_inputs.before(InputSystem));
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct InputControllerSystem;

#[derive(Resource, PartialEq)]
enum InputDevice {
    MouseKeyboard,
    Gamepad,
}

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct GlobalInput {
    pub toggle_fullscreen: bool,
    pub toggle_debug: bool,
    pub toggle_grid_debug_visuals: bool,
}

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct MenuInput {
    pub confirm: bool,
    pub restart: bool,
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
}

impl BitOrAssign for GlobalInput {
    fn bitor_assign(&mut self, rhs: Self) {
        self.toggle_fullscreen |= rhs.toggle_fullscreen;
        self.toggle_debug |= rhs.toggle_debug;
        self.toggle_grid_debug_visuals |= rhs.toggle_grid_debug_visuals;
    }
}

impl BitOrAssign for MenuInput {
    fn bitor_assign(&mut self, rhs: Self) {
        self.confirm |= rhs.confirm;
        self.restart |= rhs.restart;
    }
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
    }
}

fn reset_inputs(
    mut global_input: ResMut<GlobalInput>,
    mut menu_input: ResMut<MenuInput>,
    mut gaming_input: ResMut<GamingInput>,
) {
    *global_input = GlobalInput::default();
    *menu_input = MenuInput::default();
    *gaming_input = GamingInput::default();
}
