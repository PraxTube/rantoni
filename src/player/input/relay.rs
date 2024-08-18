use bevy::prelude::*;
use bevy_rancic::prelude::*;

use super::controller::PlayerInput;

fn toggle_fullscreen(
    player_input: Res<PlayerInput>,
    mut ev_toggle_fullscreen: EventWriter<ToggleFullscreenEvent>,
) {
    if player_input.toggle_fullscreen {
        ev_toggle_fullscreen.send(ToggleFullscreenEvent);
    }
}

fn toggle_debug_state(
    player_input: Res<PlayerInput>,
    mut ev_toggle_debug_state: EventWriter<ToggleDebugStateEvent>,
) {
    if player_input.toggle_debug {
        ev_toggle_debug_state.send(ToggleDebugStateEvent);
    }
}

fn zoom_camera(
    player_input: Res<PlayerInput>,
    mut ev_zoom_camera: EventWriter<ZoomCameraScaleEvent>,
) {
    if player_input.scroll != 0 {
        ev_zoom_camera.send(ZoomCameraScaleEvent(player_input.scroll));
    }
}

pub struct InputRelayPlugin;

impl Plugin for InputRelayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_fullscreen, toggle_debug_state, zoom_camera));
    }
}
