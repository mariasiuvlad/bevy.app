use crate::animation::CharacterAnimationPlugin;
use crate::app_state::AppState;
use crate::combat::combat_stats::StatsBundle;
use crate::input::InputPlugin;
use crate::maps::hello_world::HelloWorldPlugin;
use crate::startup::PlayerModel;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerTarget;

#[derive(Component)]
pub struct Character(pub CharacterInfo);

pub struct CharacterInfo {
    pub name: String,
}

#[derive(Component)]
pub struct PlayerCamera;

fn setup(mut commands: Commands, player_model: Res<PlayerModel>) {
    commands
        .spawn((
            SceneBundle {
                scene: player_model.0.clone(),

                transform: Transform {
                    rotation: Quat::from_rotation_y(-180.),
                    ..default()
                },
                ..default()
            },
            Player,
            StatsBundle::default(),
            Character(CharacterInfo {
                name: String::from("Player"),
            }),
        ))
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 4., -6.)
                        .looking_at(Vec3::new(0., 2., 0.), Vec3::Y),
                    camera: Camera {
                        hdr: true,
                        order: 1,
                        ..default()
                    },
                    ..default()
                })
                .insert(PlayerCamera);
        });
}

pub struct World3dPlugin;

impl Plugin for World3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HelloWorldPlugin, InputPlugin, CharacterAnimationPlugin))
            .add_systems(OnEnter(AppState::Game), setup);
    }
}
