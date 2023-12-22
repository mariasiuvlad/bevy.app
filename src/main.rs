mod app_state;
mod main_menu;

use app_state::AppState;
use bevy::prelude::*;
use main_menu::MainMenuPlugin;

fn log_state(state: Res<State<AppState>>) {
    info!("We are in the {:?} state", State::get(&state));
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((DefaultPlugins, MainMenuPlugin))
        .add_systems(OnEnter(AppState::InGame), log_state)
        .run();
}
