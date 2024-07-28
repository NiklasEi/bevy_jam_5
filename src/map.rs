use crate::loading::{ImageAssets, TILE_SIZE};
use crate::physics::GameLayer;
use crate::tank::FuelLevel;
use crate::{GameState, HEIGHT, WIDTH};
use avian2d::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_map)
            .add_systems(
                Update,
                (reload_map, toilet_sensor, update_fuel).run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_map(
    assets: Res<ImageAssets>,
    mut commands: Commands,
    images: Res<Assets<Image>>,
    fuel_level: Res<FuelLevel>,
) {
    let map = images.get(&assets.map).unwrap();
    generate_map(map, &mut commands, &assets);
    build_ship(&mut commands, &assets, &fuel_level);
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

        if value == &1 {
            commands.spawn(tile_bundle(x, y, assets));
        }
    }
}

fn build_ship(commands: &mut Commands, assets: &ImageAssets, fuel_level: &FuelLevel) {
    // leg left
    commands
        .spawn(())
        .spawn_ship_tile(107, 1, 14, assets, None)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(75, 1, 13, assets, None)
        .add_collider();
    // leg right
    commands
        .spawn(())
        .spawn_ship_tile(107, 21, 14, assets, None)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(75, 21, 13, assets, None)
        .add_collider();
    // platform
    commands
        .spawn(())
        .spawn_ship_tile(103, 1, 12, assets, None)
        .add_collider();
    for x in 2..21 {
        let index = if thread_rng().gen_bool(1. / 3.) {
            104
        } else {
            88
        };

        let mut entity = commands.spawn(());
        entity.spawn_ship_tile(index, x, 12, assets, None);
        if x != 11 {
            entity.add_collider();
        }
    }
    // exit ladders
    commands
        .spawn((Ladder, Sensor))
        .spawn_ship_tile(11, 11, 12, assets, None)
        .add_collider();
    commands
        .spawn((Ladder, Sensor))
        .spawn_ship_tile(43, 11, 13, assets, None)
        .add_collider();
    commands
        .spawn((Ladder, Sensor))
        .spawn_ship_tile(43, 11, 14, assets, None)
        .add_collider();

    commands
        .spawn(())
        .spawn_ship_tile(106, 21, 12, assets, None)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(107, 3, 13, assets, None)
        .add_collider();
    // side walls
    for y in 0..12 {
        let index = 58;
        commands
            .spawn(())
            .spawn_ship_tile(index, 2, y, assets, None)
            .add_collider();
        commands
            .spawn(())
            .spawn_ship_tile(index, 20, y, assets, None)
            .add_collider();
    }
    commands
        .spawn(())
        .spawn_ship_tile(8, 3, 0, assets, None)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(24, 3, 1, assets, None)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(40, 3, 2, assets, None)
        .add_collider();
    //toilet
    commands
        .spawn(())
        .spawn_ship_tile(28, 3, 11, assets, Some(2))
        .insert((
            Sensor,
            Toilet,
            RigidBody::Static,
            Collider::rectangle(TILE_SIZE / 2., TILE_SIZE / 2.),
            CollisionLayers::new(GameLayer::Ground, GameLayer::Player),
        ));
    commands
        .spawn(())
        .spawn_ship_tile(12, 3, 9, assets, Some(2));
    // farm
    commands
        .spawn(())
        .spawn_ship_tile(17, 6, 11, assets, None)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(18, 7, 11, assets, None)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(19, 8, 11, assets, None)
        .add_collider();

    // tank
    for y in 0..5 {
        commands
            .spawn(())
            .spawn_ship_tile(75, 19, y, assets, None)
            .add_collider();
        commands
            .spawn(())
            .spawn_ship_tile(75, 18, y, assets, None)
            .add_collider();
    }
    // tube exit
    commands
        .spawn(())
        .spawn_ship_tile(79, 19, 5, assets, None)
        .add_collider();
    commands
        .spawn(())
        .spawn_ship_tile(79, 18, 5, assets, None)
        .add_collider();
    // fuel splash
    render_fuel_tank(commands, assets, fuel_level);

    // tank input
    commands
        .spawn((TankInput, Sensor))
        .spawn_ship_tile(10, 17, 11, assets, None)
        .add_collider();
}

#[derive(Component)]
struct MapTile;
#[derive(Component)]
struct Toilet;
#[derive(Component)]
pub(crate) struct Ladder;
#[derive(Component)]
pub(crate) struct TankInput;

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
        scale: Option<usize>,
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
        scale: Option<usize>,
    ) -> &mut Self {
        self.insert((
            SpriteBundle {
                transform: {
                    let mut transform = Transform::from_xyz(
                        2. - WIDTH / 4. + TILE_SIZE * x as f32,
                        HEIGHT / 4. - TILE_SIZE * y as f32,
                        0.,
                    );
                    if let Some(scale) = scale {
                        transform.scale = Vec3::splat(scale as f32);
                        transform.translation = Vec3::new(
                            2. - WIDTH / 4.
                                + TILE_SIZE * (x - 1) as f32
                                + (TILE_SIZE * scale as f32 + TILE_SIZE) / 2.,
                            HEIGHT / 4.
                                - TILE_SIZE * (y - 1) as f32
                                - (TILE_SIZE * scale as f32 - TILE_SIZE) / 2.,
                            0.,
                        )
                    }
                    transform
                },
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

fn update_fuel(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    fuel_level: Res<FuelLevel>,
    fuel_tiles: Query<Entity, With<FuelTile>>,
) {
    if !fuel_level.is_changed() {
        return;
    }
    for entity in &fuel_tiles {
        commands.entity(entity).despawn_recursive();
    }
    render_fuel_tank(&mut commands, &assets, &fuel_level);
}

#[derive(Component)]
struct FuelTile;

fn render_fuel_tank(commands: &mut Commands, assets: &ImageAssets, fuel_level: &FuelLevel) {
    let full_blocks = (fuel_level.0.min(100.) / 20.) as usize;
    let empty = 5 - full_blocks;

    // splash
    commands
        .spawn(FuelTile)
        .spawn_ship_tile(95, 19, 6 + empty, assets, None)
        .add_collider();
    commands
        .spawn(FuelTile)
        .spawn_ship_tile(95, 18, 6 + empty, assets, None)
        .add_collider();
    // fuel
    for y in 6 + empty + 1..12 {
        commands
            .spawn(FuelTile)
            .spawn_ship_tile(45, 19, y, assets, None)
            .add_collider();
        commands
            .spawn(FuelTile)
            .spawn_ship_tile(45, 18, y, assets, None)
            .add_collider();
    }
}

fn toilet_sensor(mut query: Query<(&mut Sprite, &CollidingEntities), With<Toilet>>) {
    for (mut sprite, colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = Color::srgb(0.2, 0.7, 0.9);
        } else {
            sprite.color = Color::srgb(0.9, 0.7, 0.2);
        }
    }
}
