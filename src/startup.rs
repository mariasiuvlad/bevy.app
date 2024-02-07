use bevy::prelude::*;

use crate::{app_state::AppState, main_menu::UiFont};

pub struct StartupPlugin;

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let font_handle = asset_server.load("fonts/JetBrainsMono-Medium.ttf");
    let font_handle = asset_server.load("fonts/PixelifySans-Regular.ttf");
    commands.insert_resource(UiFont(font_handle));
}

fn check_assets_ready(
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    ui_font_handle: Res<UiFont>,
) {
    match asset_server.load_state(ui_font_handle.0.id()) {
        bevy::asset::LoadState::Loaded => {
            app_state.set(AppState::MainMenu);
        }
        bevy::asset::LoadState::Failed => todo!(),
        _ => {}
    }
}

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
