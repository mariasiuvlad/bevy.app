use std::time::Duration;

use bevy::prelude::*;

use super::{
    motion::{Motion, VelChange},
    traits::basis::{Basis, BasisContext},
    utils::ProjectionPlaneForRotation,
};

#[derive(Debug, Clone, Copy)]
pub struct SpringConfig {
    strength: f32,
    damper: f32,
}
impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            strength: 10.,
            damper: 0.8,
        }
    }
}

#[derive(Debug, Default)]
pub struct WalkMotionState {
    pub spring_force: f32,
    pub standing_offset: f32,
    airborne_timer: Option<Timer>,
}

#[derive(Copy, Clone)]
pub struct WalkMotionType {
    pub velocity: Vec3,
    pub facing: Option<Direction3d>,
    pub spring_config: SpringConfig,
    pub up: Direction3d,
    pub floating_height: f32,
    pub turning_angvel: f32,
}

impl Default for WalkMotionType {
    fn default() -> Self {
        Self {
            velocity: Default::default(),
            facing: None,
            spring_config: SpringConfig::default(),
            up: Direction3d::Y,
            floating_height: 2.0,
            turning_angvel: 5.,
        }
    }
}

impl WalkMotionType {
    fn calculate_spring_force(&self, ctx: BasisContext) -> f32 {
        match ctx.proximity_sensor_output {
            None => 0.,
            Some(output) => match output.distance > self.floating_height + 0.1 {
                true => 0.,
                false => {
                    let deviation = self.floating_height - output.distance;
                    (deviation * self.spring_config.strength)
                        - (ctx.velocity.linvel.y * self.spring_config.damper)
                }
            },
        }
    }

    fn get_torque(&self, ctx: BasisContext) -> f32 {
        let existing_angvel = ctx.velocity.angvel.dot(Vec3::from(self.up));
        match self.facing {
            None => -existing_angvel,
            Some(facing) => {
                let projection = ProjectionPlaneForRotation::from_up_using_default_forward(self.up);
                let current_forward = ctx.transform.rotation.mul_vec3(projection.forward);
                let rotation_along_up_axis =
                    projection.rotation_to_set_forward(current_forward, Vec3::from(facing));

                let desired_angvel = (rotation_along_up_axis / ctx.frame_duration)
                    .clamp(-self.turning_angvel, self.turning_angvel);

                desired_angvel - existing_angvel
            }
        }
    }
}

impl Basis for WalkMotionType {
    const NAME: &'static str = "Walk";

    type State = WalkMotionState;

    fn apply(&self, state: &mut Self::State, ctx: BasisContext, motion: &mut Motion) {
        if let Some(timer) = &mut state.airborne_timer {
            timer.tick(Duration::from_secs_f32(ctx.frame_duration));
        }

        match &mut state.airborne_timer {
            Some(_) => match &ctx.proximity_sensor_output {
                Some(sensor_output) => {
                    if sensor_output.distance <= self.floating_height + 0.05 {
                        state.airborne_timer = None;
                    }
                }
                None => {}
            },
            None => match &ctx.proximity_sensor_output {
                None => {
                    state.airborne_timer =
                        Some(Timer::new(Duration::from_millis(150), TimerMode::Once));
                }
                Some(sensor_output) => {
                    state.standing_offset = sensor_output.distance - self.floating_height
                }
            },
        }

        // horizontal movement
        let delta_velocity = (self.velocity - ctx.velocity.linvel).reject_from(Vec3::from(self.up));
        let target_velocity = match self.velocity == Vec3::ZERO {
            true => VelChange::boost(delta_velocity),
            false => VelChange::accel(delta_velocity),
        };
        let horizontal_change = if !self.is_airborne(state) {
            target_velocity
        } else {
            VelChange::ZERO
        };

        // vertical movement
        let spring_force = self.calculate_spring_force(ctx);
        let vertical_change = VelChange::boost(self.up * spring_force);

        motion.linvel = horizontal_change + vertical_change;

        let angular_change = if !self.is_airborne(state) {
            VelChange::boost(self.get_torque(ctx) * Vec3::from(self.up))
        } else {
            VelChange::boost(-ctx.velocity.angvel)
        };
        motion.angvel = angular_change;

        info!("is_airborne {:?}", self.is_airborne(state));
        // update state
        state.spring_force = spring_force;
    }

    fn is_airborne(&self, state: &Self::State) -> bool {
        state
            .airborne_timer
            .as_ref()
            .is_some_and(|timer| timer.finished())
    }

    fn displacement(&self, state: &Self::State) -> Option<Vec3> {
        match state.airborne_timer {
            None => Some(Vec3::Y * state.standing_offset),
            Some(_) => None,
        }
    }
}
