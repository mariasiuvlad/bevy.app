use crate::{
    app_state::AppState,
    world3d::{Player, PlayerTarget},
};
use bevy::prelude::*;

mod components;
mod systems;

pub use systems::setup_unitframes;

pub struct PlayerUnitframePlugin;
impl Plugin for PlayerUnitframePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::update_health_value::<Player>,
                systems::update_energy_value::<Player>,
                systems::update_health_percentage::<Player>,
                systems::update_energy_percentage::<Player>,
                systems::update_name::<Player>,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}

pub struct PlayerTargetUnitframePlugin;
impl Plugin for PlayerTargetUnitframePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::update_health_value::<PlayerTarget>,
                systems::update_energy_value::<PlayerTarget>,
                systems::update_health_percentage::<PlayerTarget>,
                systems::update_energy_percentage::<PlayerTarget>,
                systems::update_name::<PlayerTarget>,
                systems::update_display::<PlayerTarget>,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
