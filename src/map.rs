use crate::loading::{ImageAssets, TILE_SIZE};
use crate::physics::GameLayer;
use crate::{GameState, HEIGHT, WIDTH};
use avian2d::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

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
    build_ship(&mut commands, &assets);
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

fn build_ship(commands: &mut Commands, assets: &ImageAssets) {
    // leg left
    commands
        .spawn(())
        .spawn_ship_tile(107, 1, 14, assets)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(75, 1, 13, assets)
        .add_collider();
    // leg right
    commands
        .spawn(())
        .spawn_ship_tile(107, 21, 14, assets)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(75, 21, 13, assets)
        .add_collider();
    // platform
    commands
        .spawn(())
        .spawn_ship_tile(103, 1, 12, assets)
        .add_collider();
    for x in 2..21 {
        let index = if thread_rng().gen_bool(1. / 3.) {
            104
        } else {
            88
        };

        let mut entity = commands.spawn(());
        entity.spawn_ship_tile(index, x, 12, assets);
        if x != 11 {
            entity.add_collider();
        }
    }
    commands
        .spawn(())
        .spawn_ship_tile(106, 21, 12, assets)
        .add_collider();
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

trait MapCommand {
    fn spawn_ship_tile(
        &mut self,
        index: usize,
        x: usize,
        y: usize,
        assets: &ImageAssets,
    ) -> &mut Self;
    fn add_collider(&mut self) -> &mut Self;
}

impl MapCommand for EntityCommands<'_> {
    fn spawn_ship_tile(
        &mut self,
        index: usize,
        x: usize,
        y: usize,
        assets: &ImageAssets,
    ) -> &mut Self {
        self.insert((
            SpriteBundle {
                transform: Transform::from_xyz(
                    2. - WIDTH / 4. + TILE_SIZE * x as f32,
                    HEIGHT / 4. - TILE_SIZE * y as f32,
                    0.,
                ),
                texture: assets.tilemap_ship.clone(),
                ..default()
            },
            TextureAtlas {
                layout: assets.tilemap_ship_layout.clone(),
                index,
            },
        ))
    }

    fn add_collider(&mut self) -> &mut Self {
        self.insert((
            RigidBody::Static,
            Collider::rectangle(TILE_SIZE, TILE_SIZE),
            CollisionLayers::new(GameLayer::Ground, GameLayer::Player),
        ))
    }
}
