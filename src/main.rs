#![allow(clippy::type_complexity)]
mod menu;

use crate::menu::MenuPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MenuPlugin))
        .init_state::<GameState>()
        .run();
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
}
