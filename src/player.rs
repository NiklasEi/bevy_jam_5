use crate::animation::{AnimationIndices, AnimationTimer};
use crate::loading::ImageAssets;
use crate::GameState;
use avian2d::collision::{Collider, CollidingEntities};
use avian2d::math::AdjustPrecision;
use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_tnua::builtins::{TnuaBuiltinJump, TnuaBuiltinWalk};
use bevy_tnua::controller::{TnuaController, TnuaControllerBundle};
use bevy_tnua::{
    TnuaAction, TnuaActionContext, TnuaActionInitiationDirective, TnuaActionLifecycleDirective,
    TnuaActionLifecycleStatus, TnuaMotor, TnuaUserControlsSystemSet,
};
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

fn apply_controls(
    ladders: Query<&CollidingEntities, With<crate::map::Ladder>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut TnuaController>,
) {
    let Ok(mut controller) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction -= Vec3::Y;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
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
        acceleration: 400.,
        desired_forward: if direction.x > 0. { Vec3::X } else { -Vec3::X },
        air_acceleration: 200.,
        ..Default::default()
    });

    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            height: 60.0,
            ..Default::default()
        });
    }

    let climb = direction.y < 0.
        && ladders
            .iter()
            .any(|colliding_entities| !colliding_entities.is_empty());
    if controller.action_name() == Some("Ladder") || climb {
        controller.action(LadderAction {
            desired_velocity: if climb { Some(5.) } else { None },
        });
    }
}

struct LadderAction {
    pub desired_velocity: Option<f32>,
}

impl TnuaAction for LadderAction {
    const NAME: &'static str = "Ladder";
    type State = ();
    const VIOLATES_COYOTE_TIME: bool = true;

    fn apply(
        &self,
        _state: &mut Self::State,
        ctx: TnuaActionContext,
        _lifecycle_status: TnuaActionLifecycleStatus,
        motor: &mut TnuaMotor,
    ) -> TnuaActionLifecycleDirective {
        let Some(desired_velocity) = self.desired_velocity else {
            return TnuaActionLifecycleDirective::Finished;
        };
        let up = ctx.up_direction().adjust_precision();

        let velocity_on_plane = ctx
            .tracker
            .velocity
            .reject_from(ctx.up_direction().adjust_precision());

        let desired_boost = Vec3::Y * desired_velocity - velocity_on_plane;

        let walk_acceleration = desired_boost / ctx.frame_duration;

        motor.lin.cancel_on_axis(up);
        motor.lin.acceleration = walk_acceleration;

        TnuaActionLifecycleDirective::StillActive
    }

    fn initiation_decision(
        &self,
        _ctx: TnuaActionContext,
        _being_fed_for: &Stopwatch,
    ) -> TnuaActionInitiationDirective {
        TnuaActionInitiationDirective::Allow
    }
}
