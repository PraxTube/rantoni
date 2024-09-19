use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "dummy_background.png")]
    pub dummy_background: Handle<Image>,

    #[asset(path = "player/player.png")]
    pub player_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 48, tile_size_y = 48, columns = 18, rows = 3))]
    pub player_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "player/player.trickfilm.ron#idle",
            "player/player.trickfilm.ron#run",
            "player/player.trickfilm.ron#punch1",
            "player/player.trickfilm.ron#punch1recover",
            "player/player.trickfilm.ron#punch2",
            "player/player.trickfilm.ron#punch2recover",
            "player/player.trickfilm.ron#stagger",
            "player/player.trickfilm.ron#kick1",
            "player/player.trickfilm.ron#kick1recover",
            "player/player.trickfilm.ron#kick2",
            "player/player.trickfilm.ron#kick2recover",
        ),
        collection(typed)
    )]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

    // --- FONTS ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
}
