mod animations;

pub use animations::DudeAnimations;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,

    #[asset(path = "arc.png")]
    pub arc: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 6, rows = 1))]
    pub arc_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "arc.trickfilm.ron#main")]
    pub arc_animation: Handle<AnimationClip2D>,

    #[asset(
        paths(
            "dude/dude_idle.png",
            "dude/dude_run.png",
            "dude/dude_jab.png",
            "dude/dude_jab_recover.png",
            "dude/dude_punch.png",
            "dude/dude_punch_recover.png",
            "dude/dude_front_kick.png",
            "dude/dude_front_kick_recover.png",
            "dude/dude_stagger.png",
        ),
        collection(typed)
    )]
    pub dude_textures: Vec<Handle<Image>>,

    #[asset(texture_atlas_layout(tile_size_x = 100, tile_size_y = 100, columns = 30, rows = 8))]
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
            "dude/dude.trickfilm.ron#run-o0",
            "dude/dude.trickfilm.ron#run-o1",
            "dude/dude.trickfilm.ron#run-o2",
            "dude/dude.trickfilm.ron#run-o3",
            "dude/dude.trickfilm.ron#run-o4",
            "dude/dude.trickfilm.ron#run-o5",
            "dude/dude.trickfilm.ron#run-o6",
            "dude/dude.trickfilm.ron#run-o7",
            "dude/dude.trickfilm.ron#jab-o0",
            "dude/dude.trickfilm.ron#jab-o1",
            "dude/dude.trickfilm.ron#jab-o2",
            "dude/dude.trickfilm.ron#jab-o3",
            "dude/dude.trickfilm.ron#jab-o4",
            "dude/dude.trickfilm.ron#jab-o5",
            "dude/dude.trickfilm.ron#jab-o6",
            "dude/dude.trickfilm.ron#jab-o7",
            "dude/dude.trickfilm.ron#jab_recover-o0",
            "dude/dude.trickfilm.ron#jab_recover-o1",
            "dude/dude.trickfilm.ron#jab_recover-o2",
            "dude/dude.trickfilm.ron#jab_recover-o3",
            "dude/dude.trickfilm.ron#jab_recover-o4",
            "dude/dude.trickfilm.ron#jab_recover-o5",
            "dude/dude.trickfilm.ron#jab_recover-o6",
            "dude/dude.trickfilm.ron#jab_recover-o7",
            "dude/dude.trickfilm.ron#punch-o0",
            "dude/dude.trickfilm.ron#punch-o1",
            "dude/dude.trickfilm.ron#punch-o2",
            "dude/dude.trickfilm.ron#punch-o3",
            "dude/dude.trickfilm.ron#punch-o4",
            "dude/dude.trickfilm.ron#punch-o5",
            "dude/dude.trickfilm.ron#punch-o6",
            "dude/dude.trickfilm.ron#punch-o7",
            "dude/dude.trickfilm.ron#punch_recover-o0",
            "dude/dude.trickfilm.ron#punch_recover-o1",
            "dude/dude.trickfilm.ron#punch_recover-o2",
            "dude/dude.trickfilm.ron#punch_recover-o3",
            "dude/dude.trickfilm.ron#punch_recover-o4",
            "dude/dude.trickfilm.ron#punch_recover-o5",
            "dude/dude.trickfilm.ron#punch_recover-o6",
            "dude/dude.trickfilm.ron#punch_recover-o7",
            "dude/dude.trickfilm.ron#front_kick-o0",
            "dude/dude.trickfilm.ron#front_kick-o1",
            "dude/dude.trickfilm.ron#front_kick-o2",
            "dude/dude.trickfilm.ron#front_kick-o3",
            "dude/dude.trickfilm.ron#front_kick-o4",
            "dude/dude.trickfilm.ron#front_kick-o5",
            "dude/dude.trickfilm.ron#front_kick-o6",
            "dude/dude.trickfilm.ron#front_kick-o7",
            "dude/dude.trickfilm.ron#front_kick_recover-o0",
            "dude/dude.trickfilm.ron#front_kick_recover-o1",
            "dude/dude.trickfilm.ron#front_kick_recover-o2",
            "dude/dude.trickfilm.ron#front_kick_recover-o3",
            "dude/dude.trickfilm.ron#front_kick_recover-o4",
            "dude/dude.trickfilm.ron#front_kick_recover-o5",
            "dude/dude.trickfilm.ron#front_kick_recover-o6",
            "dude/dude.trickfilm.ron#front_kick_recover-o7",
            "dude/dude.trickfilm.ron#stagger-o0",
            "dude/dude.trickfilm.ron#stagger-o1",
            "dude/dude.trickfilm.ron#stagger-o2",
            "dude/dude.trickfilm.ron#stagger-o3",
            "dude/dude.trickfilm.ron#stagger-o4",
            "dude/dude.trickfilm.ron#stagger-o5",
            "dude/dude.trickfilm.ron#stagger-o6",
            "dude/dude.trickfilm.ron#stagger-o7",
        ),
        collection(typed)
    )]
    pub dude_animations: Vec<Handle<AnimationClip2D>>,

    // --- FONTS ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
}
