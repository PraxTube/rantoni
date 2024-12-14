use bevy::prelude::*;
use bevy_rancic::prelude::*;

use crate::player::input::GamingInput;

const VOLUME_DELTA: f64 = 0.05;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_main_volume,));
    }
}

fn update_main_volume(mut game_audio: ResMut<GameAudio>, gaming_input: Res<GamingInput>) {
    if gaming_input.scroll == 0 {
        return;
    }
    game_audio.increment_global_volume(gaming_input.scroll as f64 * VOLUME_DELTA);
}
