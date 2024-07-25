use crate::animation::{AnimationIndices, AnimationTimer};
use crate::loading::ImageAssets;
use crate::GameState;
use avian2d::collision::Collider;
use bevy::prelude::*;
use std::time::Duration;

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
        AnimationTimer(Timer::new(Duration::from_millis(300), TimerMode::Repeating)),
        AnimationIndices { first: 4, last: 5 },
        Collider::capsule(8., 6.0),
    ));
}
