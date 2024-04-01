use bevy::prelude::*;

use super::super::OrbitCameraEvents;

pub fn zoom_event_emitter(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_camera: EventWriter<OrbitCameraEvents>,
) {
    if keys.pressed(KeyCode::KeyO) {
        ev_camera.send(OrbitCameraEvents::Zoom(10.));
    }
    if keys.pressed(KeyCode::KeyP) {
        ev_camera.send(OrbitCameraEvents::Zoom(-10.));
    }
}
