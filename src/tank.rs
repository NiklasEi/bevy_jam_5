use crate::map::TankInput;
use crate::GameState;
use avian2d::collision::CollidingEntities;
use bevy::prelude::*;

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FuelLevel>()
            .add_systems(OnEnter(GameState::Playing), prep_tank)
            .add_systems(Update, feed_tank.run_if(in_state(GameState::Playing)));
    }
}

fn prep_tank(mut commands: Commands) {
    commands.insert_resource(FuelLevel(0.));
    commands.insert_resource(TankTimer(-1.));
}

#[derive(Resource)]
struct TankTimer(f64);

fn feed_tank(
    query: Query<&CollidingEntities, With<TankInput>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tank: ResMut<FuelLevel>,
    mut tank_timer: ResMut<TankTimer>,
    time: Res<Time>,
) {
    for colliding_entities in &query {
        if !colliding_entities.is_empty() && keyboard.pressed(KeyCode::KeyF) {
            if time.elapsed_seconds_f64() - tank_timer.0 > 1. {
                tank.0 += 10.;
                tank_timer.0 = time.elapsed_seconds_f64();
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct FuelLevel(pub(crate) f32);
