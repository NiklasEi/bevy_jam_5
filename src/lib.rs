mod animation;
mod loading;
mod player;

use crate::animation::SpriteAnimationPlugin;
use crate::loading::LoadingPlugin;
use crate::player::PlayerPlugin;
use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::*;

pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 600.;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Prepare,
    Playing,
    Restart,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((PlayerPlugin, LoadingPlugin, SpriteAnimationPlugin))
            .add_systems(Update, start_level.run_if(in_state(GameState::Prepare)));
    }
}

fn start_level(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
