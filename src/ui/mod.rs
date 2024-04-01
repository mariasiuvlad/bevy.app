mod fps;
mod nameplate;
mod nodes;
pub mod style;
mod unitframes;

use bevy::prelude::*;

use crate::app_state::AppState;
pub use fps::FpsPlugin;
use nameplate::NameplatePlugin;

use self::unitframes::{setup_unitframes, PlayerTargetUnitframePlugin, PlayerUnitframePlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_unitframes)
            .add_plugins((
                NameplatePlugin,
                PlayerUnitframePlugin,
                PlayerTargetUnitframePlugin,
            ));
    }
}
