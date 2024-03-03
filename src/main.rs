mod animated_bundle;
mod animation;
mod app_state;
mod combat;
mod components;
mod input;
mod main_menu;
mod maps;
mod movement;
mod movement_new;
mod nameplate;
mod player_camera;
mod startup;
mod texture;
mod ui;
mod ui_style;
mod world3d;

use app_state::AppState;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use combat::CombatPlugin;
use main_menu::MainMenuPlugin;
use movement::MovementPlugin;
use movement_new::MovementNewPlugin;
use nameplate::NameplatePlugin;
use player_camera::OrbitCameraPlugin;
use startup::StartupPlugin;
use ui::fps::FpsPlugin;
use ui::UiPlugin;
use world3d::World3dPlugin;
mod mouse;

#[macro_export]
macro_rules! get_single {
    ($q:expr) => {
        match $q.get_single() {
            Ok(m) => m,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_single_mut {
    ($q:expr) => {
        match $q.get_single_mut() {
            Ok(m) => m,
            _ => return,
        }
    };
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FpsPlugin,
            StartupPlugin,
            MainMenuPlugin,
            CombatPlugin,
            MovementPlugin,
            MovementNewPlugin,
            UiPlugin,
            NameplatePlugin,
            World3dPlugin,
            OrbitCameraPlugin,
        ))
        .run();
}
