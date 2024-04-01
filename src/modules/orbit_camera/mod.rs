use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::app_state::AppState;

mod systems;

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

pub struct OrbitCameraPlugin;
impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrbitCameraEvents>().add_systems(
            Update,
            (
                systems::orbit_event_emitter,
                systems::zoom_event_emitter,
                systems::camera_event_handler,
                systems::transform_controller,
                systems::follow_controller,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
