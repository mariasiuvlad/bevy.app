use std::f32::consts::PI;

use bevy::{asset::LoadState, pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::{
    animated_bundle::{AnimatedModelBundle, AnimationState, AnimationStates, ModelAnimations},
    app_state::AppState,
    combat::combat_stats::StatsBundle,
    components::meta::Name,
    world3d::{Character, CharacterInfo, Player, PlayerCamera},
};

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Goblin;

#[derive(Resource)]
pub struct HeroModel(pub Handle<Scene>);

#[derive(Resource)]
pub struct GoblinModel(pub Handle<Scene>);

#[derive(Resource)]
pub struct Animations<T: Component>(T, pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hero_model = asset_server.load("models/hero.glb#Scene0");
    let goblin_model = asset_server.load("models/goblin.glb#Scene0");

    commands.insert_resource(HeroModel(hero_model.clone()));
    commands.insert_resource(GoblinModel(goblin_model.clone()));

    commands.insert_resource(Animations(
        Goblin,
        vec![
            asset_server.load("models/goblin.glb#Animation0"),
            asset_server.load("models/goblin.glb#Animation1"),
            asset_server.load("models/goblin.glb#Animation2"),
            asset_server.load("models/goblin.glb#Animation3"),
            asset_server.load("models/goblin.glb#Animation4"),
            asset_server.load("models/goblin.glb#Animation5"),
        ],
    ));

    commands.insert_resource(Animations(
        Hero,
        vec![
            asset_server.load("models/hero.glb#Animation0"),
            asset_server.load("models/hero.glb#Animation1"),
            asset_server.load("models/hero.glb#Animation2"),
            asset_server.load("models/hero.glb#Animation3"),
            asset_server.load("models/hero.glb#Animation4"),
            asset_server.load("models/hero.glb#Animation5"),
        ],
    ));

    commands.insert_resource(AssetsLoading(vec![
        hero_model.clone_weak().untyped(),
        goblin_model.clone_weak().untyped(),
    ]));
}

fn check_assets_ready(
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    let y = loading.0.iter().map(|x| asset_server.load_state(x.id()));

    if y.filter(|x| *x != LoadState::Loaded)
        .collect::<Vec<_>>()
        .len()
        == 0
    {
        app_state.set(AppState::Game);
    }
}

fn setup_hero(
    mut commands: Commands,
    hero_model: Res<HeroModel>,
    hero_animations: Res<Animations<Hero>>,
) {
    commands
        .spawn((
            Name::new("Hero"),
            Player,
            Hero,
            AnimatedModelBundle {
                animation_state: AnimationState(AnimationStates::Idle),
                scene: SceneBundle {
                    scene: hero_model.0.clone(),
                    transform: Transform::from_xyz(3.0, 0.0, -8.0),
                    ..default()
                },
                animations: ModelAnimations::from_vec(&hero_animations.1),
            },
            StatsBundle::default(),
            Character(CharacterInfo {
                name: String::from("Hero"),
            }),
        ))
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 4., -6.)
                        .looking_at(Vec3::new(0., 2., 0.), Vec3::Y),
                    camera: Camera {
                        order: 1,
                        ..default()
                    },
                    ..default()
                })
                .insert(PlayerCamera);
        });
}

fn setup_goblin(
    mut commands: Commands,
    goblin_model: Res<GoblinModel>,
    goblin_animations: Res<Animations<Goblin>>,
) {
    commands.spawn((
        Goblin,
        AnimatedModelBundle {
            animation_state: AnimationState(AnimationStates::Idle),
            animations: ModelAnimations::from_vec(&goblin_animations.1),
            scene: SceneBundle {
                scene: goblin_model.0.clone(),
                transform: Transform::from_xyz(3.0, 0.0, 8.0),
                ..default()
            },
        },
        StatsBundle::default(),
        Character(CharacterInfo {
            name: String::from("Goblin"),
        }),
    ));
}

fn setup_lights(mut commands: Commands) {
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 9000000.0,
    //         range: 100.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(24.0, 16.0, 24.0),
    //     ..default()
    // });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });
}

pub struct AnimatedModelsPlugin;
impl Plugin for AnimatedModelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadingGame), load_assets)
            .add_systems(
                Update,
                check_assets_ready.run_if(in_state(AppState::LoadingGame)),
            )
            .add_systems(
                OnEnter(AppState::Game),
                (setup_world, setup_lights, setup_hero, setup_goblin),
            );
    }
}
