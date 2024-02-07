use crate::app_state::AppState;
use crate::input::keyboard_input;
use crate::maps;
use crate::texture;
use bevy::prelude::*;

/// A marker component for our shapes so we can query them separately from the ground plane
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
pub struct CharacterUI(pub Entity);

#[derive(Component)]
pub struct PlayerTargetUI;

#[derive(Component)]
pub struct HasCharacterUI;

#[derive(Component)]
pub struct PlayerCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(texture::debug::uv())),
        ..default()
    });

    // let shapes = [
    //     meshes.add(shape::Cube::default().into()),
    //     meshes.add(shape::Box::default().into()),
    //     meshes.add(shape::Capsule::default().into()),
    //     meshes.add(shape::Torus::default().into()),
    //     meshes.add(shape::Cylinder::default().into()),
    //     meshes.add(shape::Icosphere::default().try_into().unwrap()),
    //     meshes.add(shape::UVSphere::default().into()),
    // ];

    let shape = meshes.add(shape::Capsule::default().into());

    commands
        .spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
            Player,
            Character(CharacterInfo {
                name: String::from("Player"),
            }),
        ))
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 9., 12.0)
                        .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
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
        app.add_systems(OnEnter(AppState::Game), (maps::hello_world::setup, setup))
            .add_systems(Update, keyboard_input.run_if(in_state(AppState::Game)));
    }
}
