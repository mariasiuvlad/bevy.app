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
use nameplate::NameplatePlugin;
use startup::StartupPlugin;
use ui::fps::FpsPlugin;
use ui::UiPlugin;
use world3d::World3dPlugin;

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
            UiPlugin,
            NameplatePlugin,
            World3dPlugin,
        ))
        .run();
}
