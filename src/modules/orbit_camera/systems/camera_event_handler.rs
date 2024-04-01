use bevy::prelude::*;

use super::super::{OrbitCamera, OrbitCameraEvents};

pub fn camera_event_handler(
    time: Res<Time>,
    mut ev_camera: EventReader<OrbitCameraEvents>,
    mut camera_query: Query<&mut OrbitCamera, With<Camera>>,
) {
    for mut camera in camera_query.iter_mut() {
        for event in ev_camera.read() {
            match event {
                OrbitCameraEvents::Orbit(delta) => {
                    camera.x -= delta.x * camera.rotate_sensitivity * time.delta_seconds();
                    camera.y -= delta.y * camera.rotate_sensitivity * time.delta_seconds();
                    camera.y = camera
                        .y
                        .max(*camera.pitch_range.start())
                        .min(*camera.pitch_range.end());
                }
                OrbitCameraEvents::Zoom(value) => camera.distance += value * time.delta_seconds(),
            }
        }
    }
}
