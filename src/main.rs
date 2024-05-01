use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::*;

use app_state::AppState;
use input::PlayerKeyboardInputPlugin;
use main_menu::MainMenuPlugin;
use maps::physics_platformer::PhysicsPlatformerPlugin;
use modules::{
    brain::BrainPlugin, character_controller::CharacterControllerPlugin, combat::CombatPlugin,
    orbit_camera::OrbitCameraPlugin,
};
use startup::StartupPlugin;
use ui::UiPlugin;

mod app_state;
mod components;
mod input;
mod main_menu;
mod maps;
mod modules;
mod mouse;
mod startup;
mod ui;
mod window_config;
mod world3d;

#[macro_export]
macro_rules! get_single {
    ($q:expr) => {
        match $q.get_single() {
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
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(window_config::get_window_config()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            EguiPlugin,
            StartupPlugin,
            MainMenuPlugin,
            BrainPlugin,
            CombatPlugin,
            UiPlugin,
            OrbitCameraPlugin,
            CharacterControllerPlugin::default(),
            PhysicsPlatformerPlugin,
            PlayerKeyboardInputPlugin,
        ))
        .run();
}
