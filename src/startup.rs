use bevy::prelude::*;

use crate::{app_state::AppState, main_menu::UiFont};

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("fonts/MadimiOne-Regular.ttf");

    commands.insert_resource(UiFont(handle));
}

fn check_assets_ready(
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
            .add_systems(OnEnter(AppState::Startup), load_assets)
            .add_systems(
                Update,
                check_assets_ready.run_if(in_state(AppState::Startup)),
            );
    }
}
