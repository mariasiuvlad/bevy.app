mod animated_bundle;
mod animation;
mod app_state;
mod combat;
mod input;
mod main_menu;
mod maps;
mod movement;
mod nameplate;
mod startup;
mod texture;
mod ui;
mod ui_style;
mod world3d;

use app_state::AppState;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use combat::CombatPlugin;
use main_menu::MainMenuPlugin;
use movement::MovementPlugin;
use nameplate::NameplatePlugin;
use startup::StartupPlugin;
use ui::UiPlugin;
use world3d::World3dPlugin;

fn _log_fps(diagnostics: Res<DiagnosticsStore>) {
    if let Some(value) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        info!("FPS: {}", value)
    }
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            // FrameTimeDiagnosticsPlugin::default(),
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            StartupPlugin,
            MainMenuPlugin,
            CombatPlugin,
            MovementPlugin,
            UiPlugin,
            NameplatePlugin,
            World3dPlugin,
        ))
        .run();
}
