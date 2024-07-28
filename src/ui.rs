use crate::tank::FuelLevel;
use crate::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_ui)
            .add_systems(Update, update_tank_ui.run_if(in_state(GameState::Playing)));
    }
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::LinearRgba(LinearRgba::new(1., 1., 1., 0.6))),
            style: Style {
                width: Val::Px(100.),
                height: Val::Px(30.),
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            },
            ..default()
        })
        .with_children(|node| {
            node.spawn(
                TextBundle::from_section(
                    "Tank:",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                }),
            );
            node.spawn((
                TextBundle::from_section(
                    "0%",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    align_content: AlignContent::End,
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                }),
                TankUi,
            ));
        });
}

#[derive(Component)]
struct TankUi;

fn update_tank_ui(mut tank_ui: Query<&mut Text, With<TankUi>>, fuel_level: Res<FuelLevel>) {
    if fuel_level.is_changed() {
        tank_ui.single_mut().sections[0].value = format!("{}%", fuel_level.0.min(100.).round())
    }
}
