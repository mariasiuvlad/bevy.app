mod app_state;
mod combat;
mod input;
mod main_menu;
mod maps;
mod player_ui;
mod startup;
mod texture;
mod ui_style;
mod world3d;
mod world3d_ui;

use app_state::AppState;
use bevy::prelude::*;
use combat::CombatPlugin;
use main_menu::MainMenuPlugin;
use player_ui::PlayerUiPlugin;
use startup::StartupPlugin;
use world3d::World3dPlugin;
use world3d_ui::World3dUiPlugin;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            StartupPlugin,
            MainMenuPlugin,
            CombatPlugin,
            PlayerUiPlugin,
            World3dUiPlugin,
            World3dPlugin,
        ))
        .run();
}
