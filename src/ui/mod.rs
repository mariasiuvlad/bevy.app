use crate::app_state::AppState;
use bevy::prelude::*;

use self::unitframe::{setup_unitframes, PlayerNameplatePlugin, PlayerTargetNameplatePlugin};

pub mod common;
pub mod fps;
mod unitframe;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_unitframes)
            .add_plugins((PlayerNameplatePlugin, PlayerTargetNameplatePlugin));
    }
}
