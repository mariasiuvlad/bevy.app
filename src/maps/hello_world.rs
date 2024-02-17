use bevy::prelude::*;

use crate::{
    combat::{combat_stats::StatsBundle, status_effect::thorns::ThornsEffect},
    texture,
    world3d::{Character, CharacterInfo},
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let shapes = [
    //     meshes.add(shape::Cube::default().into()),
    //     meshes.add(shape::Box::default().into()),
    //     meshes.add(shape::Capsule::default().into()),
    //     meshes.add(shape::Torus::default().into()),
    //     meshes.add(shape::Cylinder::default().into()),
    //     meshes.add(shape::Icosphere::default().try_into().unwrap()),
    //     meshes.add(shape::UVSphere::default().into()),
    // ];

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(texture::debug::uv())),
        ..default()
    });

    let shape = meshes.add(shape::Cube::default().into());

    commands.spawn((
        PbrBundle {
            mesh: shape.clone(),
            material: debug_material.clone(),
            transform: Transform::from_xyz(-3.0, 1.0, -8.0),
            ..default()
        },
        StatsBundle::default(),
        Character(CharacterInfo {
            name: String::from("Rak'thar"),
        }),
    ));

    commands.spawn((
        PbrBundle {
            mesh: shape.clone(),
            material: debug_material.clone(),
            transform: Transform::from_xyz(3.0, 1.0, -8.0),
            ..default()
        },
        StatsBundle::default(),
        ThornsEffect {
            timer: Timer::from_seconds(600., TimerMode::Once),
        },
        Character(CharacterInfo {
            name: String::from("Mog'sha"),
        }),
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });
}
