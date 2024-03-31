use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    app_state::AppState,
    character_controller::CharacterController,
    combat::combat_stats::StatsBundle,
    components::{cleanup, meta::Name},
    get_single,
    mouse::{cursor_grab, cursor_ungrab},
    plugins::orbit_camera,
    world3d::{Character, CharacterInfo, Player, PlayerCamera},
};

#[derive(Resource)]
pub struct LevelZero(pub Handle<Mesh>);

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_zero = asset_server.load("models/level1.glb#Mesh0/Primitive0");
    commands.insert_resource(LevelZero(level_zero));
}

fn setup_hero(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Capsule3d::new(1., 0.5));
    let material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::hex("#698996").unwrap().into(),
        metallic: 1.0,
        perceptual_roughness: 0.5,
        ..default()
    });

    commands
        .spawn((
            Name::new("Hero"),
            Player,
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule_y(0.5, 1.),
            CharacterController::default(),
            Velocity::default(),
            ExternalImpulse::default(),
            ExternalForce::default(),
            ColliderMassProperties::Density(0.0),
            AdditionalMassProperties::Mass(1.0),
            TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)),
            StatsBundle::default(),
            Character(CharacterInfo {
                name: String::from("Hero"),
            }),
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::default(),
                ..default()
            });
        });
}

fn setup_npc(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Capsule3d::new(1., 0.2));
    let material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::hex("#EBBAB9").unwrap().into(),
        metallic: 1.0,
        perceptual_roughness: 0.5,
        ..default()
    });

    commands
        .spawn((
            Name::new("Eve"),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule_y(0.2, 1.),
            CharacterController::default(),
            Velocity::default(),
            ExternalImpulse::default(),
            ExternalForce::default(),
            ColliderMassProperties::Density(0.0),
            AdditionalMassProperties::Mass(1.0),
            TransformBundle::from(Transform::from_xyz(-5.0, 2.0, 0.0)),
            StatsBundle::default(),
            Character(CharacterInfo {
                name: String::from("Eve"),
            }),
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::default(),
                ..default()
            });
        });
}

fn setup_lights(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

fn setup_world(
    mut commands: Commands,
    level_zero: Res<LevelZero>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = level_zero.0.clone();
    let mesh = meshes.get(mesh_handle.clone_weak()).unwrap();

    commands
        .spawn(PbrBundle {
            mesh: mesh_handle.clone_weak(),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("#ffd891").unwrap(),
                metallic: 0.5,
                perceptual_roughness: 0.5,
                ..default()
            }),
            transform: Transform::default(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                RigidBody::Fixed,
                TransformBundle::default(),
                Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(),
            ));
        });
}

pub fn setup_player_camera(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player_entity = get_single!(player_query);

    commands.spawn((
        Name::new("Player Camera"),
        cleanup::LevelUnload,
        PlayerCamera,
        orbit_camera::OrbitCamera::new(10., Vec3::ZERO, Some(player_entity)),
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
    ));
}

pub struct PhysicsPlatformerPlugin;
impl Plugin for PhysicsPlatformerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Startup), load_assets)
            .add_systems(
                OnEnter(AppState::Game),
                (
                    cursor_grab,
                    setup_world,
                    setup_lights,
                    setup_hero,
                    setup_npc,
                    setup_player_camera.after(setup_hero),
                ),
            )
            .add_systems(OnExit(AppState::Game), cursor_ungrab);
    }
}
