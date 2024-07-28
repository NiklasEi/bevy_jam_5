mod animation;
mod loading;
mod map;
mod physics;
mod player;
mod tank;
mod ui;

use crate::animation::SpriteAnimationPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::player::PlayerPlugin;
use crate::tank::TankPlugin;
use crate::ui::UiPlugin;
use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;

pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 600.;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
    Restart,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(Gravity(Vector::Y * -98.1))
            .add_plugins((
                PhysicsPlugins::default().with_length_unit(10.),
                PlayerPlugin,
                LoadingPlugin,
                SpriteAnimationPlugin,
                MapPlugin,
                TnuaControllerPlugin::default(),
                TnuaAvian2dPlugin::default(),
                UiPlugin,
                TankPlugin,
            ))
            .add_systems(Startup, spawn_camera);
        #[cfg(debug_assertions)]
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    commands.spawn(camera);
}
