mod audio_bar;
mod screen_fade;
mod splash_screen;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            screen_fade::ScreenFadeUiPlugin,
            audio_bar::AudioBarPlugin,
            splash_screen::SplashScreenPlugin,
        ));
    }
}
