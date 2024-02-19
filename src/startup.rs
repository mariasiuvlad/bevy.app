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

fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let font_handle = asset_server.load("fonts/JetBrainsMono-Medium.ttf");
    let font_handle = asset_server.load("fonts/PixelifySans-Regular.ttf");

    commands.insert_resource(UiFont(font_handle.clone()));
}

fn check_fonts_ready(
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    ui_font: Res<UiFont>,
) {
    match asset_server.load_state(ui_font.0.id()) {
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
            .add_systems(OnEnter(AppState::Startup), load_fonts)
            .add_systems(
                Update,
                check_fonts_ready.run_if(in_state(AppState::Startup)),
            );
    }
}
