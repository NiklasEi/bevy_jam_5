use crate::loading::{ImageAssets, TILE_SIZE};
use crate::physics::GameLayer;
use crate::{GameState, HEIGHT, WIDTH};
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_map);
    }
}

fn spawn_map(assets: Res<ImageAssets>, mut commands: Commands, images: Res<Assets<Image>>) {
    let map = images.get(&assets.map).unwrap();
    for (tile, value) in map.data.iter().step_by(4).enumerate() {
        let x = (tile % 23) as f32;
        let y = (tile / 23) as f32;

        match value {
            &1 => {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            2. - WIDTH / 4. + TILE_SIZE * x,
                            HEIGHT / 4. - TILE_SIZE * y,
                            0.,
                        ),
                        texture: assets.tilemap.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: assets.tilemap_layout.clone(),
                        index: 122,
                    },
                    RigidBody::Static,
                    Collider::rectangle(TILE_SIZE, TILE_SIZE),
                    CollisionLayers::new(GameLayer::Ground, GameLayer::Player),
                ));
            }
            _ => (),
        };
    }
}
