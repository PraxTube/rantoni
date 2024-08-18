use bevy::prelude::*;
use bevy_rancic::prelude::*;

use crate::player::input::PlayerInput;

const VOLUME_DELTA: f64 = 0.05;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_main_volume,));
    }
}

fn update_main_volume(mut game_audio: ResMut<GameAudio>, player_input: Res<PlayerInput>) {
    if player_input.scroll == 0 {
        return;
    }
    game_audio.increment_global_volume(player_input.scroll as f64 * VOLUME_DELTA);
}
