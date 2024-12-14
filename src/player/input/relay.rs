use bevy::prelude::*;

use crate::world::{ToggleDebugStateEvent, ToggleFullscreenEvent, ZoomCameraScaleEvent};

use super::{GamingInput, GlobalInput};

fn toggle_fullscreen(
    global_input: Res<GlobalInput>,
    mut ev_toggle_fullscreen: EventWriter<ToggleFullscreenEvent>,
) {
    if global_input.toggle_fullscreen {
        ev_toggle_fullscreen.send(ToggleFullscreenEvent);
    }
}

fn toggle_debug_state(
    global_input: Res<GlobalInput>,
    mut ev_toggle_debug_state: EventWriter<ToggleDebugStateEvent>,
) {
    if global_input.toggle_debug {
        ev_toggle_debug_state.send(ToggleDebugStateEvent);
    }
}

fn zoom_camera(
    gaming_input: Res<GamingInput>,
    mut ev_zoom_camera: EventWriter<ZoomCameraScaleEvent>,
) {
    if gaming_input.scroll != 0 {
        ev_zoom_camera.send(ZoomCameraScaleEvent(gaming_input.scroll));
    }
}

pub struct InputRelayPlugin;

impl Plugin for InputRelayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_fullscreen, toggle_debug_state, zoom_camera));
    }
}
