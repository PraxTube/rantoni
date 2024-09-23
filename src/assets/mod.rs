mod animations;

pub use animations::DudeAnimations;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,

    #[asset(
        paths(
            "dude/dude_idle.png",
            "dude/dude_jab.png",
            "dude/dude_punch.png",
            "dude/dude_run.png",
        ),
        collection(typed)
    )]
    pub dude_textures: Vec<Handle<Image>>,

    #[asset(texture_atlas_layout(tile_size_x = 100, tile_size_y = 100, columns = 40, rows = 8))]
    pub dude_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "dude/dude.trickfilm.ron#idle-o0",
            "dude/dude.trickfilm.ron#idle-o1",
            "dude/dude.trickfilm.ron#idle-o2",
            "dude/dude.trickfilm.ron#idle-o3",
            "dude/dude.trickfilm.ron#idle-o4",
            "dude/dude.trickfilm.ron#idle-o5",
            "dude/dude.trickfilm.ron#idle-o6",
            "dude/dude.trickfilm.ron#idle-o7",
            "dude/dude.trickfilm.ron#jab-o0",
            "dude/dude.trickfilm.ron#jab-o1",
            "dude/dude.trickfilm.ron#jab-o2",
            "dude/dude.trickfilm.ron#jab-o3",
            "dude/dude.trickfilm.ron#jab-o4",
            "dude/dude.trickfilm.ron#jab-o5",
            "dude/dude.trickfilm.ron#jab-o6",
            "dude/dude.trickfilm.ron#jab-o7",
            "dude/dude.trickfilm.ron#punch-o0",
            "dude/dude.trickfilm.ron#punch-o1",
            "dude/dude.trickfilm.ron#punch-o2",
            "dude/dude.trickfilm.ron#punch-o3",
            "dude/dude.trickfilm.ron#punch-o4",
            "dude/dude.trickfilm.ron#punch-o5",
            "dude/dude.trickfilm.ron#punch-o6",
            "dude/dude.trickfilm.ron#punch-o7",
            "dude/dude.trickfilm.ron#run-o0",
            "dude/dude.trickfilm.ron#run-o1",
            "dude/dude.trickfilm.ron#run-o2",
            "dude/dude.trickfilm.ron#run-o3",
            "dude/dude.trickfilm.ron#run-o4",
            "dude/dude.trickfilm.ron#run-o5",
            "dude/dude.trickfilm.ron#run-o6",
            "dude/dude.trickfilm.ron#run-o7",
        ),
        collection(typed)
    )]
    pub dude_animations: Vec<Handle<AnimationClip2D>>,

    // --- FONTS ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
}
