use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{
    motion::{Motion, VelChange},
    motion_type::MotionType,
    proximity_sensor::ProximitySensorOutput,
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

#[derive(Debug, Clone, Copy)]
pub struct MotionTypeContext {
    pub frame_duration: f32,
    pub proximity_sensor_output: Option<ProximitySensorOutput>,
    pub transform: Transform,
    pub velocity: Velocity,
}

#[derive(Debug, Default)]
pub struct WalkMotionState {
    pub spring_force: f32,
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
    fn calculate_spring_force(&self, ctx: MotionTypeContext) -> f32 {
        match ctx.proximity_sensor_output {
            None => 0.,
            Some(output) => match output.distance > self.floating_height + 0.1 {
                true => 0.,
                false => {
                    info!("{:?}", output.distance);
                    let deviation = self.floating_height - output.distance;
                    (deviation * self.spring_config.strength)
                        - (ctx.velocity.linvel.y * self.spring_config.damper)
                }
            },
        }
    }

    fn get_torque(&self, ctx: MotionTypeContext) -> f32 {
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

impl MotionType for WalkMotionType {
    const NAME: &'static str = "Walk";

    type State = WalkMotionState;

    fn apply(&self, state: &mut Self::State, ctx: MotionTypeContext, motion: &mut Motion) {
        let delta_velocity = (self.velocity - ctx.velocity.linvel).reject_from(Vec3::from(self.up));
        let target_velocity = match self.velocity == Vec3::ZERO {
            true => VelChange::boost(delta_velocity),
            false => VelChange::accel(delta_velocity),
        };

        let spring_force = self.calculate_spring_force(ctx);

        motion.linvel = target_velocity + VelChange::boost(self.up * spring_force);
        motion.angvel = VelChange::boost(self.get_torque(ctx) * Vec3::from(self.up));

        // update state
        state.spring_force = spring_force;
    }
}
