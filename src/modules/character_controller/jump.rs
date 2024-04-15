use bevy::prelude::*;

use crate::modules::character_controller::traits::action_type::ActionLifecycle;

use super::{
    motion::{Motion, VelChange},
    traits::action_type::{
        ActionInitiationDirective, ActionLifecycleDirective, ActionType, ActionTypeContext,
    },
    WalkMotionType,
};

#[derive(Default, Debug)]
pub enum JumpActionTypeState {
    #[default]
    NoJump,
    StartingJump,
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

impl ActionType for JumpActionType {
    fn apply(
        &self,
        state: &mut Self::State,
        ctx: ActionTypeContext,
        lifecycle: ActionLifecycle,
        motion: &mut Motion,
    ) -> ActionLifecycleDirective {
        let impulse = if lifecycle.just_started() {
            *state = JumpActionTypeState::StartingJump;
            VelChange::impulse(Vec3::Y * 5.)
        } else {
            VelChange::ZERO
        };

        let current_motion_type = ctx.concrete_motion_type::<WalkMotionType>();

        let boost = if let Some((_, motion_state)) = current_motion_type {
            VelChange::boost(Vec3::Y * -motion_state.spring_force)
        } else {
            VelChange::ZERO
        };

        motion.linvel += impulse + boost;

        if ctx.velocity.linvel.y > 0. || lifecycle.just_started() {
            ActionLifecycleDirective::Active
        } else {
            ActionLifecycleDirective::Finished
        }
    }

    const NAME: &'static str = "Jump";

    type State = JumpActionTypeState;

    fn initiation_decision(&self, ctx: ActionTypeContext) -> ActionInitiationDirective {
        if ctx.motion_type.is_airborne() {
            ActionInitiationDirective::Reject
        } else {
            ActionInitiationDirective::Allow
        }
    }
}
