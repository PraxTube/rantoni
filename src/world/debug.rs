use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::input::{GlobalInput, InputControllerSystem};

/// Indicates whether the game is currently in debug mode.
/// This can be used for just debugging info to the player (developer),
/// or it can also act as a trigger to allow cheats etc.
#[derive(Resource, Default)]
pub struct DebugState {
    pub active: bool,
    pub grid_visuals_active: bool,
}

fn toggle_debug_state(global_input: Res<GlobalInput>, mut debug_state: ResMut<DebugState>) {
    if global_input.toggle_debug {
        debug_state.active = !debug_state.active;
    }
}

fn toggle_rapier_debug(
    mut debug_context: ResMut<DebugRenderContext>,
    debug_state: Res<DebugState>,
) {
    if debug_context.enabled != debug_state.active {
        debug_context.enabled = debug_state.active;
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugState>().add_systems(
            PreUpdate,
            (toggle_debug_state, toggle_rapier_debug).after(InputControllerSystem),
        );
    }
}
