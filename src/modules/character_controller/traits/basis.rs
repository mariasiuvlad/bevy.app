use std::any::Any;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::modules::character_controller::{
    motion::Motion, proximity_sensor::ProximitySensorOutput,
};

#[derive(Debug, Clone, Copy)]
pub struct BasisContext {
    pub frame_duration: f32,
    pub proximity_sensor_output: Option<ProximitySensorOutput>,
    pub transform: Transform,
    pub velocity: Velocity,
    pub gravity: Vec3,
}

pub trait Basis: 'static + Send + Sync {
    const NAME: &'static str;

    type State: Default + Send + Sync;

    fn apply(&self, state: &mut Self::State, ctx: BasisContext, motion: &mut Motion);

    fn displacement(&self, state: &Self::State) -> Option<Vec3>;

    fn is_airborne(&self, state: &Self::State) -> bool;
}

pub trait DynamicBasis: Send + Sync + Any + 'static {
    #[doc(hidden)]
    fn as_any(&self) -> &dyn Any;

    #[doc(hidden)]
    fn as_mut_any(&mut self) -> &mut dyn Any;

    #[doc(hidden)]
    fn apply(&mut self, ctx: BasisContext, motion: &mut Motion);

    fn displacement(&self) -> Option<Vec3>;

    fn is_airborne(&self) -> bool;
}

pub(crate) struct BoxableBasis<B: Basis> {
    pub(crate) input: B,
    pub(crate) state: B::State,
}

impl<B: Basis> BoxableBasis<B> {
    pub(crate) fn new(motion_type: B) -> Self {
        Self {
            input: motion_type,
            state: Default::default(),
        }
    }
}

impl<B: Basis> DynamicBasis for BoxableBasis<B> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn apply(&mut self, ctx: BasisContext, motion: &mut Motion) {
        self.input.apply(&mut self.state, ctx, motion)
    }

    fn displacement(&self) -> Option<Vec3> {
        self.input.displacement(&self.state)
    }

    fn is_airborne(&self) -> bool {
        self.input.is_airborne(&self.state)
    }
}
