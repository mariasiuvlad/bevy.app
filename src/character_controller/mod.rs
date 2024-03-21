use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::orbit_camera::OrbitCamera;

const GRAVITY: f32 = -9.81;

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

fn calculate_spring_force(spring_config: &SpringConfig, deviation: f32, velocity: f32) -> f32 {
    (-GRAVITY + (deviation * spring_config.strength)) - (velocity * spring_config.damper)
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
    pub translation: Vec3,
    pub jump_timer: Timer,
    pub floating_config: FloatingConfig,
    pub cast_shape_result: Option<(Entity, Toi)>,
    pub forward_reference: Option<Entity>,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            translation: Vec3::default(),
            floating_config: FloatingConfig::default(),
            jump_timer: Timer::new(Duration::from_millis(150), TimerMode::Once),
            cast_shape_result: None,
            forward_reference: None,
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

fn horizontal_input_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut CharacterController>,
) {
    for mut controller in q.iter_mut() {
        controller.translation.x = 0.;
        controller.translation.z = 0.;
        if keys.pressed(KeyCode::KeyW) {
            controller.translation.x += 1.;
        }
        if keys.pressed(KeyCode::KeyA) {
            controller.translation.z += -1.;
        }
        if keys.pressed(KeyCode::KeyS) {
            controller.translation.x += -1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            controller.translation.z += 1.;
        }
    }
}

fn vertical_input_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut CharacterController, &mut ExternalImpulse)>,
) {
    for (mut ctr, mut impulse) in q.iter_mut() {
        if keys.just_pressed(KeyCode::Space) && ctr.is_grounded() {
            ctr.jump_timer.reset();
            impulse.impulse = Vec3::Y * 5.;
        }
    }
}

fn tick_timers(time: Res<Time>, mut q: Query<&mut CharacterController>) {
    for mut ctr in q.iter_mut() {
        ctr.jump_timer.tick(time.delta());
    }
}

fn cast_shape_manager(
    mut q: Query<(&mut CharacterController, &Transform, &Collider)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut controller, transform, collider) in q.iter_mut() {
        let shape_vel: Vec3 = -transform.local_y().normalize();
        let mut collider = collider.clone();
        collider.set_scale(Vec3::new(0.9, 1.0, 0.9), 10);

        controller.cast_shape_result = rapier_context.cast_shape(
            transform.translation,
            transform.rotation,
            shape_vel,
            &collider,
            1.5,
            true,
            QueryFilter::only_fixed(),
        );
    }
}

fn floating_controller(mut q: Query<(&CharacterController, &mut ExternalForce, &Velocity)>) {
    for (controller, mut force, velocity) in q.iter_mut() {
        let spring_force = match controller.cast_shape_result {
            None => 0.,
            Some((_, toi)) => match controller.is_grounded() {
                false => 0.,
                true => controller.get_computed_spring_force(toi.toi, velocity.linvel.y),
            },
        };

        if !controller.is_jumping() {
            force.force.y = spring_force;
        }
    }
}

fn movement_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut ctr_q: Query<(&CharacterController, &mut Velocity)>,
    transform_q: Query<&Transform, With<OrbitCamera>>,
) {
    let speed = match keys.pressed(KeyCode::KeyC) {
        true => 1.,
        false => 5.,
    };

    for (controller, mut velocity) in ctr_q.iter_mut() {
        let normalized_movement = controller.translation.normalize_or_zero();
        let final_movement = match transform_q.get_single() {
            Err(_) => normalized_movement,
            Ok(ref_transform) => {
                ref_transform.forward() * normalized_movement.x
                    + ref_transform.local_x() * normalized_movement.z
            }
        };

        if controller.is_grounded() {
            velocity.linvel.x = final_movement.x * speed;
            velocity.linvel.z = final_movement.z * speed;
        }
    }
}

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (floating_controller, movement_controller))
            .add_systems(
                Update,
                (
                    cast_shape_manager,
                    vertical_input_controller,
                    horizontal_input_controller,
                    tick_timers,
                    // debug_controller,
                ),
            );
        info!("CharacterControllerPlugin::build()");
    }
}
