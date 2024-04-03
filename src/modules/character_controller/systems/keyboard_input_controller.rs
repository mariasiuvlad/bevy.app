use super::super::CharacterController;
use crate::world3d::Player;
use bevy::prelude::*;

pub fn keyboard_input_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut CharacterController, With<Player>>,
) {
    for mut ctr in q.iter_mut() {
        ctr.transform.translation.x = 0.;
        ctr.transform.translation.z = 0.;
        if keys.pressed(KeyCode::KeyW) {
            ctr.transform.translation.x += 1.;
        }
        if keys.pressed(KeyCode::KeyA) {
            ctr.transform.translation.z += -1.;
        }
        if keys.pressed(KeyCode::KeyS) {
            ctr.transform.translation.x += -1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            ctr.transform.translation.z += 1.;
        }
    }
}
