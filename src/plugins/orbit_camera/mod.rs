use std::ops::RangeInclusive;

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{app_state::AppState, world3d::CharacterTarget};

#[derive(Event)]
pub enum OrbitCameraEvents {
    Orbit(Vec2),
    Zoom(f32),
}

#[derive(Component)]
pub struct OrbitCamera {
    pub x: f32,
    pub y: f32,
    pub pitch_range: RangeInclusive<f32>,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub subject: Option<Entity>,
}
impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            x: 0.0,
            y: std::f32::consts::FRAC_PI_2,
            pitch_range: 0.5..=1.7,
            distance: 5.0,
            center: Vec3::ZERO,
            rotate_sensitivity: 0.5,
            zoom_sensitivity: 0.8,
            subject: None,
        }
    }
}
impl OrbitCamera {
    pub fn new(dist: f32, center: Vec3, subject: Option<Entity>) -> OrbitCamera {
        OrbitCamera {
            distance: dist,
            center,
            subject,
            ..Self::default()
        }
    }
}

pub fn emit_motion_events(
    mut ev_mouse_motion: EventReader<MouseMotion>,
    mut ev_camera: EventWriter<OrbitCameraEvents>,
) {
    let mut delta = Vec2::ZERO;
    for event in ev_mouse_motion.read() {
        delta += event.delta;
    }

    ev_camera.send(OrbitCameraEvents::Orbit(delta));
}

pub fn emit_keyboard_events(
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

pub fn handle_events(
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

pub fn update_transform(
    mut query: Query<(&OrbitCamera, &mut Transform), (Changed<OrbitCamera>, With<Camera>)>,
    target_q: Query<&Transform, (With<CharacterTarget>, Without<Camera>)>,
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
        // t.look_at(target, Vec3::Y);
    }
}

pub fn follow_subject(
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

pub struct OrbitCameraPlugin;
impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrbitCameraEvents>().add_systems(
            Update,
            (
                emit_motion_events,
                emit_keyboard_events,
                handle_events,
                update_transform,
                follow_subject,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
