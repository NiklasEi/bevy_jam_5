use crate::animation::{AnimationIndices, AnimationTimer};
use crate::loading::ImageAssets;
use crate::GameState;
use avian2d::collision::Collider;
use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJump, TnuaBuiltinWalk};
use bevy_tnua::controller::{TnuaController, TnuaControllerBundle};
use bevy_tnua::TnuaUserControlsSystemSet;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, apply_controls.in_set(TnuaUserControlsSystemSet));
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
        TnuaControllerBundle::default(),
        RigidBody::Dynamic,
    ));
}

fn apply_controls(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut TnuaController>) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction -= Vec3::Y;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 200.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 12.,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 50.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}
