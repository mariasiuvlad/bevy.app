use crate::character_controller::CharacterController;
use crate::world3d::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn keyboard_input_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut CharacterController, &mut ExternalImpulse), With<Player>>,
) {
    for (mut ctr, mut impulse) in q.iter_mut() {
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
        if keys.just_pressed(KeyCode::Space) && ctr.is_grounded() {
            ctr.jump_timer.reset();
            impulse.impulse = Vec3::Y * 5.;
        }
    }
}
