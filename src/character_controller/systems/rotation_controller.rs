use crate::character_controller::CharacterController;
use crate::modules::orbit_camera::OrbitCamera;
use bevy::prelude::*;

pub fn rotation_controller(
    mut ctr_q: Query<(&mut CharacterController, &mut Transform)>,
    transform_q: Query<&Transform, (With<OrbitCamera>, Without<CharacterController>)>,
) {
    for (mut ctr, mut t) in ctr_q.iter_mut() {
        match transform_q.get_single() {
            Err(_) => {}
            Ok(transform) => {
                ctr.transform.rotation = transform.rotation;
            }
        }

        if ctr.transform.translation != Vec3::ZERO {
            let mut facing = ctr.transform.forward() * ctr.transform.translation.x
                + ctr.transform.local_x() * ctr.transform.translation.z;
            facing.y = 0.;

            t.look_to(facing, Vec3::Y);
        }
    }
}
