use bevy::prelude::*;

use crate::{
    app_state::AppState,
    combat::{combat_stats::StatsBundle, status_effect::thorns::ThornsEffect},
    startup::{Animations, GoblinModel, PlayerModel},
    texture,
    world3d::{Character, CharacterInfo, Player, PlayerCamera},
};

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let attack_animation = asset_server.load("models/x_bot.glb#Animation0"); // attack
    let backpedal_animation = asset_server.load("models/x_bot.glb#Animation1"); // backpedal
    let flinch_animation = asset_server.load("models/x_bot.glb#Animation2"); // flinch
    let idle_animation = asset_server.load("models/x_bot.glb#Animation3"); // idle
    let run_animation = asset_server.load("models/x_bot.glb#Animation4"); // run
    let walk_animation = asset_server.load("models/x_bot.glb#Animation5"); // walk
    let player_model = asset_server.load("models/x_bot.glb#Scene0");

    commands.insert_resource(PlayerModel(player_model.clone()));
    commands.insert_resource(GoblinModel(player_model.clone()));
    commands.insert_resource(Animations {
        attack: attack_animation,
        backpedal: backpedal_animation,
        idle: idle_animation,
        flinch: flinch_animation,
        run: run_animation,
        walk: walk_animation,
    });
}

pub fn setup_world(
    mut commands: Commands,
    goblin_model: Res<GoblinModel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(texture::debug::uv())),
        ..default()
    });

    let _shape = meshes.add(shape::Cube::default().into());

    commands.spawn((
        SceneBundle {
            scene: goblin_model.0.clone(),
            transform: Transform::from_xyz(-3.0, 0.0, -8.0),
            ..default()
        },
        StatsBundle::default(),
        Character(CharacterInfo {
            name: String::from("Rak'thar"),
        }),
    ));

    commands.spawn((
        SceneBundle {
            scene: goblin_model.0.clone(),
            transform: Transform::from_xyz(3.0, 0.0, -8.0),
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

pub fn setup_player(mut commands: Commands, player_model: Res<PlayerModel>) {
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

pub struct HelloWorldMapPlugin;

impl Plugin for HelloWorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Startup), load_assets)
            .add_systems(OnEnter(AppState::Game), (setup_world, setup_player));
    }
}
