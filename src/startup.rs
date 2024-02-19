use bevy::prelude::*;

use crate::{app_state::AppState, main_menu::UiFont};

#[derive(Resource)]
pub struct Animations {
    pub attack: Handle<AnimationClip>,
    pub backpedal: Handle<AnimationClip>,
    pub idle: Handle<AnimationClip>,
    pub flinch: Handle<AnimationClip>,
    pub run: Handle<AnimationClip>,
    pub walk: Handle<AnimationClip>,
}

#[derive(Resource)]
pub struct PlayerModel(pub Handle<Scene>);

#[derive(Resource)]
pub struct GoblinModel(pub Handle<Scene>);

#[derive(Resource)]
struct AssetsLoading(Vec<UntypedHandle>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let font_handle = asset_server.load("fonts/JetBrainsMono-Medium.ttf");
    let font_handle = asset_server.load("fonts/PixelifySans-Regular.ttf");
    let attack_animation = asset_server.load("models/x_bot.glb#Animation0"); // attack
    let backpedal_animation = asset_server.load("models/x_bot.glb#Animation1"); // backpedal
    let flinch_animation = asset_server.load("models/x_bot.glb#Animation2"); // flinch
    let idle_animation = asset_server.load("models/x_bot.glb#Animation3"); // idle
    let run_animation = asset_server.load("models/x_bot.glb#Animation4"); // run
    let walk_animation = asset_server.load("models/x_bot.glb#Animation5"); // walk
    let player_model = asset_server.load("models/x_bot.glb#Scene0");

    commands.insert_resource(PlayerModel(player_model.clone()));
    commands.insert_resource(GoblinModel(player_model.clone()));

    commands.insert_resource(UiFont(font_handle.clone()));

    commands.insert_resource(Animations {
        attack: attack_animation,
        backpedal: backpedal_animation,
        idle: idle_animation,
        flinch: flinch_animation,
        run: run_animation,
        walk: walk_animation,
    });
}

fn check_assets_ready(
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    player_model: Res<PlayerModel>,
) {
    match asset_server.load_state(player_model.0.id()) {
        bevy::asset::LoadState::Loaded => {
            app_state.set(AppState::MainMenu);
        }
        bevy::asset::LoadState::Failed => todo!(),
        _ => {}
    }
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFont>()
            .add_systems(OnEnter(AppState::Startup), load_assets)
            .add_systems(
                Update,
                check_assets_ready.run_if(in_state(AppState::Startup)),
            );
    }
}
