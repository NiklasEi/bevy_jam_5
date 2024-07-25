use crate::loading::ImageAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset: Res<ImageAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: asset.tilemap_character.clone(),
            ..default()
        },
        TextureAtlas {
            layout: asset.tilemap_character_layout.clone(),
            index: 5,
        },
    ));
}
