use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component)]
pub struct UiFps;

use crate::{app_state::AppState, main_menu::UiFont};

pub fn setup(mut commands: Commands, font: Res<UiFont>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 0,
            ..default()
        },
        ..default()
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                right: Val::Px(0.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                UiFps,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font.0.clone_weak(),
                        font_size: 16.0,
                        ..default()
                    },
                ),
            ));
        });
}

pub fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<UiFps>>,
) {
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        if let Ok(mut fps_text) = fps_text_query.get_single_mut() {
            fps_text.sections[0].value = format!("fps: {:.0}", value);
        }
    }
}

pub struct FpsPlugin;
impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(Update, update_fps);
    }
}
