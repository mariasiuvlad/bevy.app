//! Plays animations from a skinned glTF.

use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::{
    app_state::AppState,
    startup::{Animations, PlayerModel},
};

pub fn setup(mut commands: Commands, monkey_model: Res<PlayerModel>) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 15.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Fox
    commands.spawn(SceneBundle {
        scene: monkey_model.0.clone(),
        ..default()
    });
}

// Once the scene is loaded, start the animation
pub fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.idle.clone_weak()).repeat();
    }
}

pub struct HelloFoxPlugin;

impl Plugin for HelloFoxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        // .add_plugins(DefaultPlugins)
        .add_systems(OnEnter(AppState::Game), setup)
        .add_systems(Update, setup_scene_once_loaded);
    }
}
