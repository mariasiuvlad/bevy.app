use bevy::prelude::*;

use super::super::OrbitCamera;

pub fn follow_controller(
    transform_query: Query<&Transform>,
    mut camera_query: Query<&mut OrbitCamera>,
) {
    for mut camera in camera_query.iter_mut() {
        match camera.subject {
            Some(e) => {
                if let Ok(transform) = transform_query.get(e) {
                    camera.center = camera.center.lerp(transform.translation + Vec3::Y, 0.1);
                }
            }
            None => return,
        }
    }
}
