use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

const GRAVITY: f32 = -9.81;

fn calculate_spring_force(spring_config: &SpringConfig, deviation: f32, velocity: f32) -> f32 {
    (-GRAVITY + (deviation * spring_config.strength)) - (velocity * spring_config.damper)
}

#[derive(Debug)]
pub struct SpringConfig {
    strength: f32,
    damper: f32,
}
impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            strength: 1000.,
            damper: 100.,
        }
    }
}

#[derive(Debug)]
pub struct FloatingConfig {
    ride_spring: SpringConfig,
    ride_height: f32,
}

impl Default for FloatingConfig {
    fn default() -> Self {
        Self {
            ride_spring: SpringConfig::default(),
            ride_height: 0.5,
        }
    }
}

#[derive(Component, Debug)]
pub struct CharacterController {
    pub transform: Transform,
    pub jump_timer: Timer,
    pub floating_config: FloatingConfig,
    pub cast_shape_result: Option<(Entity, Toi)>,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            floating_config: FloatingConfig::default(),
            jump_timer: Timer::new(Duration::from_millis(150), TimerMode::Once),
            cast_shape_result: None,
        }
    }
}

impl CharacterController {
    pub fn is_jumping(&self) -> bool {
        !self.jump_timer.finished()
    }

    pub fn is_grounded(&self) -> bool {
        match self.cast_shape_result {
            None => false,
            Some((_, toi)) => toi.toi <= self.floating_config.ride_height + 0.3,
        }
    }

    pub fn get_computed_spring_force(&self, distance: f32, relative_velocity: f32) -> f32 {
        let deviation = self.floating_config.ride_height - distance;
        calculate_spring_force(
            &self.floating_config.ride_spring,
            deviation,
            relative_velocity,
        )
    }
}
