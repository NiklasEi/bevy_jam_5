use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(image(sampler = nearest))]
    #[asset(path = "tilemap-characters.png")]
    pub tilemap_character: Handle<Image>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "tilemap-backgrounds.png")]
    pub tilemap_backgrounds: Handle<Image>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "tilemap.png")]
    pub tilemap: Handle<Image>,

    #[asset(texture_atlas_layout(
        tile_size_x = 24,
        tile_size_y = 24,
        columns = 9,
        rows = 3,
        padding_x = 1,
        padding_y = 1
    ))]
    pub tilemap_character_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 24,
        tile_size_y = 24,
        columns = 8,
        rows = 3,
        padding_x = 1,
        padding_y = 1
    ))]
    pub tilemap_backgrounds_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 18,
        tile_size_y = 18,
        columns = 20,
        rows = 9,
        padding_x = 1,
        padding_y = 1
    ))]
    pub tilemap_layout: Handle<TextureAtlasLayout>,
}
