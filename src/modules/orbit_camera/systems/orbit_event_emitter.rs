use bevy::{input::mouse::MouseMotion, prelude::*};

use super::super::OrbitCameraEvents;

pub fn orbit_event_emitter(
    mut ev_mouse_motion: EventReader<MouseMotion>,
    mut ev_camera: EventWriter<OrbitCameraEvents>,
) {
    let mut delta = Vec2::ZERO;
    for event in ev_mouse_motion.read() {
        delta += event.delta;
    }

    ev_camera.send(OrbitCameraEvents::Orbit(delta));
}
