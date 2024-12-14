mod audio_bar;
mod game_over;
mod health;
mod screen_fade;
mod splash_screen;

pub use screen_fade::FadeScreen;

use bevy::{prelude::*, window::WindowResized};

use crate::DEFAULT_WINDOW_WIDTH;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            screen_fade::ScreenFadeUiPlugin,
            game_over::GameOverUiPlugin,
            audio_bar::AudioBarPlugin,
            splash_screen::SplashScreenPlugin,
            health::UiHealthPlugin,
        ))
        .add_systems(Update, scale_ui);
    }
}

fn scale_ui(mut ui_scale: ResMut<UiScale>, mut ev_window_resized: EventReader<WindowResized>) {
    for ev in ev_window_resized.read() {
        ui_scale.0 = ev.width / DEFAULT_WINDOW_WIDTH;
    }
}
