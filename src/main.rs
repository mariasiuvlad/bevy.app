mod animated_bundle;
mod animation;
mod app_state;
mod combat;
mod components;
mod input;
mod main_menu;
mod maps;
mod movement;
mod nameplate;
mod orbit_camera;
mod startup;
mod texture;
mod ui;
mod ui_style;
mod world3d;

use app_state::AppState;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::window::PresentMode;
use combat::CombatPlugin;
use main_menu::MainMenuPlugin;
use movement::MovementNewPlugin;
use nameplate::NameplatePlugin;
use orbit_camera::OrbitCameraPlugin;
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
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "I am a window!".into(),
                        name: Some("bevy.app".into()),
                        // resolution: (500., 300.).into(),
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        present_mode: PresentMode::AutoVsync,
                        // // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        // prevent_default_event_handling: false,
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        // This will spawn an invisible window
                        // The window will be made visible in the make_visible() system after 3 frames.
                        // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }),
            FpsPlugin,
            StartupPlugin,
            MainMenuPlugin,
            CombatPlugin,
            MovementNewPlugin,
            UiPlugin,
            NameplatePlugin,
            World3dPlugin,
            OrbitCameraPlugin,
        ))
        .run();
}
