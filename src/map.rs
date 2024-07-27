use crate::loading::{ImageAssets, TILE_SIZE};
use crate::physics::GameLayer;
use crate::{GameState, HEIGHT, WIDTH};
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_map)
            .add_systems(Update, reload_map.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_map(assets: Res<ImageAssets>, mut commands: Commands, images: Res<Assets<Image>>) {
    let map = images.get(&assets.map).unwrap();
    generate_map(map, &mut commands, &assets);
}

fn reload_map(
    assets: Res<ImageAssets>,
    mut asset_reload: EventReader<AssetEvent<Image>>,
    mut commands: Commands,
    images: Res<Assets<Image>>,
    map_tiles: Query<Entity, With<MapTile>>,
) {
    for event in asset_reload.read() {
        if event.is_modified(&assets.map) {
            let map = images.get(&assets.map).unwrap();
            for entity in &map_tiles {
                commands.entity(entity).despawn_recursive();
            }
            generate_map(map, &mut commands, &assets);
        }
    }
}

fn generate_map(image: &Image, commands: &mut Commands, assets: &ImageAssets) {
    for (tile, value) in image.data.iter().step_by(4).enumerate() {
        let x = tile % 23;
        let y = tile / 23;

        match value {
            &1 => {
                commands.spawn(tile_bundle(x, y, assets));
            }
            _ => (),
        };
    }
}

#[derive(Component)]
struct MapTile;

fn tile_bundle(x: usize, y: usize, assets: &ImageAssets) -> impl Bundle {
    (
        SpriteBundle {
            transform: Transform::from_xyz(
                2. - WIDTH / 4. + TILE_SIZE * x as f32,
                HEIGHT / 4. - TILE_SIZE * y as f32,
                0.,
            ),
            texture: assets.tilemap.clone(),
            ..default()
        },
        TextureAtlas {
            layout: assets.tilemap_layout.clone(),
            index: 122,
        },
        MapTile,
        RigidBody::Static,
        Collider::rectangle(TILE_SIZE, TILE_SIZE),
        CollisionLayers::new(GameLayer::Ground, GameLayer::Player),
    )
}
