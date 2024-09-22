mod animations;

pub use animations::DudeAnimations;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,

    #[asset(path = "dude/player.png")]
    pub player_texture: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 48, tile_size_y = 48, columns = 18, rows = 3))]
    pub dude_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "dude/dude.trickfilm.ron#idle",
            "dude/dude.trickfilm.ron#run",
            "dude/dude.trickfilm.ron#attack-light1",
            "dude/dude.trickfilm.ron#recover-light1",
            "dude/dude.trickfilm.ron#attack-light2",
            "dude/dude.trickfilm.ron#recover-light2",
            "dude/dude.trickfilm.ron#attack-light3",
            "dude/dude.trickfilm.ron#recover-light3",
            "dude/dude.trickfilm.ron#attack-heavy1",
            "dude/dude.trickfilm.ron#recover-heavy1",
            "dude/dude.trickfilm.ron#attack-heavy2",
            "dude/dude.trickfilm.ron#recover-heavy2",
            "dude/dude.trickfilm.ron#attack-heavy3",
            "dude/dude.trickfilm.ron#recover-heavy3",
            "dude/dude.trickfilm.ron#stagger-normal",
            "dude/dude.trickfilm.ron#stagger-flying",
        ),
        collection(typed)
    )]
    pub dude_animations: Vec<Handle<AnimationClip2D>>,

    // --- FONTS ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
}
