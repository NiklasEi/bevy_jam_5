#![allow(clippy::type_complexity)]
mod menu;

use crate::menu::MenuPlugin;
use bevy_jam_5::{GamePlugin, GameState};

use bevy::prelude::*;
use bevy_jam_5::{HEIGHT, WIDTH};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Re-Cycles".to_string(),
                        resolution: (WIDTH, HEIGHT).into(),
                        canvas: Some("#bevy".to_owned()),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
            MenuPlugin,
            GamePlugin,
        ))
        .run();
}
