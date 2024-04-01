use bevy::prelude::*;

use crate::world3d::PlayerTarget;

use super::super::OrbitCamera;

pub fn transform_controller(
    mut query: Query<(&OrbitCamera, &mut Transform), (Changed<OrbitCamera>, With<Camera>)>,
    target_q: Query<&Transform, (With<PlayerTarget>, Without<Camera>)>,
) {
    for (camera, mut t) in query.iter_mut() {
        let (angle, target) = match target_q.get_single() {
            Ok(target_transform) => (
                Transform::from_translation(camera.center)
                    .looking_at(target_transform.translation, Vec3::Y)
                    .forward()
                    * -1.,
                target_transform.translation,
            ),
            Err(_) => (
                Quat::from_axis_angle(Vec3::Y, camera.x)
                    * Quat::from_axis_angle(-Vec3::X, camera.y)
                    * Vec3::Y,
                camera.center,
            ),
        };

        t.translation = t
            .translation
            .lerp(angle * camera.distance + camera.center + Vec3::Y, 0.1);

        t.rotation = t
            .rotation
            .slerp(t.looking_at(target, Vec3::Y).rotation, 0.5);
    }
}
