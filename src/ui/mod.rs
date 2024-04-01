use crate::app_state::AppState;
use bevy::prelude::*;
use nameplate::NameplatePlugin;

use self::unitframe::{setup_unitframes, PlayerTargetUnitframePlugin, PlayerUnitframePlugin};

pub mod common;
pub mod fps;
mod nameplate;
pub mod style;
mod unitframe;

pub use fps::FpsPlugin;

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
