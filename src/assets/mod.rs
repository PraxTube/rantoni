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
            "dude/dude.trickfilm.ron#punch1",
            "dude/dude.trickfilm.ron#punch1recover",
            "dude/dude.trickfilm.ron#punch2",
            "dude/dude.trickfilm.ron#punch2recover",
            "dude/dude.trickfilm.ron#stagger-normal",
            "dude/dude.trickfilm.ron#kick1",
            "dude/dude.trickfilm.ron#kick1recover",
            "dude/dude.trickfilm.ron#kick2",
            "dude/dude.trickfilm.ron#kick2recover",
            "dude/dude.trickfilm.ron#punch3",
            "dude/dude.trickfilm.ron#punch3recover",
            "dude/dude.trickfilm.ron#stagger-flying",
            "dude/dude.trickfilm.ron#kick3",
            "dude/dude.trickfilm.ron#kick3recover",
        ),
        collection(typed)
    )]
    pub dude_animations: Vec<Handle<AnimationClip2D>>,

    // --- FONTS ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
}
