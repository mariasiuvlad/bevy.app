use crate::{
    app_state::AppState,
    world3d::{Player, PlayerTarget},
};
use bevy::prelude::*;

use self::unitframe::{
    setup_unitframes, toggle_ui, update_energy_percentage, update_energy_value,
    update_health_percentage, update_health_value, update_name,
};

mod common;
mod unitframe;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_unitframes)
            .add_systems(
                Update,
                (
                    update_health_value::<Player>,
                    update_energy_value::<Player>,
                    update_health_percentage::<Player>,
                    update_energy_percentage::<Player>,
                    update_name::<Player>,
                    update_health_value::<PlayerTarget>,
                    update_energy_value::<PlayerTarget>,
                    update_health_percentage::<PlayerTarget>,
                    update_energy_percentage::<PlayerTarget>,
                    update_name::<PlayerTarget>,
                    toggle_ui::<PlayerTarget>,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
