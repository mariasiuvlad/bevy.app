use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{
    motion::{Motion, VelChange},
    motion_type::{BoxableMotionType, DynamicMotionType, MotionType},
    proximity_sensor::ProximitySensorOutput,
    walk::MotionTypeContext,
    WalkMotionType,
};

pub struct ActionTypeContext<'a> {
    pub frame_duration: f32,
    pub proximity_sensor_output: Option<ProximitySensorOutput>,
    pub transform: Transform,
    pub velocity: Velocity,
    pub motion_type: &'a dyn DynamicMotionType,
    pub gravity: Vec3,
}

impl<'a> ActionTypeContext<'a> {
    pub fn concrete_motion_type<M: MotionType>(&self) -> Option<(&M, &M::State)> {
        let boxable_motion_type: &BoxableMotionType<M> =
            self.motion_type.as_any().downcast_ref()?;
        Some((&boxable_motion_type.input, &boxable_motion_type.state))
    }

    pub fn as_motion_type_context(&self) -> MotionTypeContext {
        MotionTypeContext {
            frame_duration: self.frame_duration,
            velocity: self.velocity,
            proximity_sensor_output: self.proximity_sensor_output,
            transform: self.transform,
            gravity: self.gravity,
        }
    }
}

#[derive(Default)]
pub struct JumpActionState {}

pub trait ActionType {
    fn apply(&self, ctx: ActionTypeContext, motion: &mut Motion);
}

#[derive(Copy, Clone)]
pub struct JumpActionType {
    pub velocity: Vec3,
}

impl Default for JumpActionType {
    fn default() -> Self {
        Self {
            velocity: Default::default(),
        }
    }
}

impl JumpActionType {}

impl ActionType for JumpActionType {
    fn apply(&self, ctx: ActionTypeContext, motion: &mut Motion) {
        let current_motion_type = ctx.concrete_motion_type::<WalkMotionType>();
        let spring_force: f32 = if let Some((_, state)) = current_motion_type {
            state.spring_force
        } else {
            0.
        };

        let accel = 20. - ctx.gravity.y - ctx.velocity.linvel.y;

        let vel_change = VelChange {
            boost: Vec3::Y * -spring_force,
            accel: Vec3::Y * accel,
        };

        motion.linvel += vel_change;
    }
}
