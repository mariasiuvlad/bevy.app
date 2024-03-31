use bevy::prelude::*;

use super::systems;
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (systems::floating_controller, systems::movement_controller),
        )
        .add_systems(
            Update,
            (
                systems::cast_shape_manager,
                systems::rotation_controller,
                systems::input_controller,
                systems::tick_timers,
            ),
        );
    }
}
