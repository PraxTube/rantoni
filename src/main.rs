#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_range_loop
)]

mod assets;
mod audio;
mod dude;
mod enemy;
mod player;
mod ui;
mod world;

pub use assets::GameAssets;
pub type GameRng = rand_xoshiro::Xoshiro256PlusPlus;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode, WindowResolution};

use bevy_asset_loader::prelude::*;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::Animation2DPlugin;
use bevy_tweening::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    TransitionLevel,
    Ending,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
            ShapePlugin,
            ParticleSystemPlugin,
            Animation2DPlugin,
            TweeningPlugin,
            RancicPlugin,
        ))
        .insert_resource(Msaa::Off)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Gaming)
                .load_collection::<GameAssets>(),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((
            world::WorldPlugin,
            ui::UiPlugin,
            audio::GameAudioPlugin,
            dude::StatePlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            assets::AssetPlugin,
        ))
        .run();
}
