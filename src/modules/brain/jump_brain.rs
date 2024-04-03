use bevy::prelude::*;

use crate::modules::actions::jump::JumpEvent;

#[derive(Component)]
pub struct JumpBrain;

pub fn jump_brain_controller(
    mut ev_jump: EventWriter<JumpEvent>,
    mut brain_query: Query<Entity, With<JumpBrain>>,
) {
    for brain_handle in brain_query.iter_mut() {
        ev_jump.send(JumpEvent(brain_handle));
    }
}

pub struct JumpBrainPlugin;
impl Plugin for JumpBrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, jump_brain_controller);
    }
}
