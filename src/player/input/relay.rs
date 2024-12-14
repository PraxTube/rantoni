use bevy::prelude::*;
use bevy_rancic::prelude::*;

use super::GamingInput;

fn toggle_fullscreen(
    gaming_input: Res<GamingInput>,
    mut ev_toggle_fullscreen: EventWriter<ToggleFullscreenEvent>,
) {
    if gaming_input.toggle_fullscreen {
        ev_toggle_fullscreen.send(ToggleFullscreenEvent);
    }
}

fn toggle_debug_state(
    gaming_input: Res<GamingInput>,
    mut ev_toggle_debug_state: EventWriter<ToggleDebugStateEvent>,
) {
    if gaming_input.toggle_debug {
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
