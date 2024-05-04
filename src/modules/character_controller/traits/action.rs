use std::any::Any;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::modules::character_controller::{
    motion::Motion, proximity_sensor::ProximitySensorOutput,
};

use super::basis::{Basis, BasisContext, BoxableBasis, DynamicBasis};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ActionInitiationDirective {
    Allow,
    Reject,
}

#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub enum ActionLifecycle {
    #[default]
    Started,
    StillFed,
    NoLongerFed,
}

impl ActionLifecycle {
    pub fn _directive_simple(&self) -> ActionLifecycleDirective {
        match self {
            ActionLifecycle::NoLongerFed => ActionLifecycleDirective::Finished,
            ActionLifecycle::Started | ActionLifecycle::StillFed => {
                ActionLifecycleDirective::Active
            }
        }
    }

    pub fn _just_started(&self) -> bool {
        match self {
            ActionLifecycle::Started => true,
            _ => false,
        }
    }

    pub fn _is_active(&self) -> bool {
        match self {
            ActionLifecycle::NoLongerFed => false,
            _ => true,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ActionLifecycleDirective {
    Active,
    Finished,
}

pub trait Action: 'static + Send + Sync {
    const NAME: &'static str;

    type State: Default + Send + Sync;

    fn apply(
        &self,
        state: &mut Self::State,
        ctx: ActionContext,
        lifecycle: ActionLifecycle,
        motion: &mut Motion,
    ) -> ActionLifecycleDirective;

    fn initiation_decision(&self, ctx: ActionContext) -> ActionInitiationDirective;
}

pub trait DynamicActionType: 'static + Send + Sync + Any {
    #[doc(hidden)]
    fn as_any(&self) -> &dyn Any;

    #[doc(hidden)]
    fn as_mut_any(&mut self) -> &mut dyn Any;

    #[doc(hidden)]
    fn apply(
        &mut self,
        ctx: ActionContext,
        lifecycle: ActionLifecycle,
        motion: &mut Motion,
    ) -> ActionLifecycleDirective;

    fn initiation_decision(&self, ctx: ActionContext) -> ActionInitiationDirective;
}

pub(crate) struct BoxableActionType<A: Action> {
    pub(crate) input: A,
    pub(crate) state: A::State,
}

impl<A: Action> BoxableActionType<A> {
    pub(crate) fn new(action_type: A) -> Self {
        Self {
            input: action_type,
            state: Default::default(),
        }
    }
}

impl<A: Action> DynamicActionType for BoxableActionType<A> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn apply(
        &mut self,
        ctx: ActionContext,
        lifecycle: ActionLifecycle,
        motion: &mut Motion,
    ) -> ActionLifecycleDirective {
        self.input.apply(&mut self.state, ctx, lifecycle, motion)
    }

    fn initiation_decision(&self, ctx: ActionContext) -> ActionInitiationDirective {
        self.input.initiation_decision(ctx)
    }
}

pub struct ActionContext<'a> {
    pub frame_duration: f32,
    pub proximity_sensor_output: Option<ProximitySensorOutput>,
    pub transform: Transform,
    pub velocity: Velocity,
    pub gravity: Vec3,
    pub motion_type: &'a dyn DynamicBasis,
}

impl<'a> ActionContext<'a> {
    pub fn concrete_motion_type<M: Basis>(&self) -> Option<(&M, &M::State)> {
        let boxable_motion_type: &BoxableBasis<M> = self.motion_type.as_any().downcast_ref()?;
        Some((&boxable_motion_type.input, &boxable_motion_type.state))
    }

    pub fn _as_motion_type_context(&self) -> BasisContext {
        BasisContext {
            frame_duration: self.frame_duration,
            velocity: self.velocity,
            proximity_sensor_output: self.proximity_sensor_output,
            transform: self.transform,
            gravity: self.gravity,
        }
    }
}
