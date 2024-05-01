use bevy::prelude::*;

use crate::modules::character_controller::traits::action::ActionLifecycle;

use super::{
    motion::{Motion, VelChange},
    traits::action::{Action, ActionContext, ActionInitiationDirective, ActionLifecycleDirective},
    WalkMotionType,
};

#[derive(Default, Debug)]
pub enum JumpActionState {
    #[default]
    Started,
    Active,
    Finished,
}

#[derive(Clone, Copy)]
pub struct JumpAction {
    pub velocity: Vec3,
}

impl Default for JumpAction {
    fn default() -> Self {
        Self {
            velocity: Default::default(),
        }
    }
}

impl Action for JumpAction {
    fn apply(
        &self,
        state: &mut Self::State,
        ctx: ActionContext,
        lifecycle: ActionLifecycle,
        motion: &mut Motion,
    ) -> ActionLifecycleDirective {
        let current_motion_type = ctx.concrete_motion_type::<WalkMotionType>();
        if let Some((_, motion_state)) = current_motion_type {
            let boost = (Vec3::Y * -motion_state.spring_force).max(Vec3::ZERO);
            motion.linvel += VelChange::boost(boost);
        }

        match state {
            JumpActionState::Started => {
                if lifecycle.just_started() {
                    motion.linvel += VelChange::impulse(Vec3::Y * 5.);
                }
                if ctx.motion_type.is_airborne() {
                    *state = JumpActionState::Active;
                }
                ActionLifecycleDirective::Active
            }
            JumpActionState::Active => {
                if !ctx.motion_type.is_airborne() {
                    *state = JumpActionState::Finished;
                }
                ActionLifecycleDirective::Active
            }
            JumpActionState::Finished => ActionLifecycleDirective::Finished,
        }
    }

    const NAME: &'static str = "Jump";

    type State = JumpActionState;

    fn initiation_decision(&self, ctx: ActionContext) -> ActionInitiationDirective {
        if ctx.motion_type.is_airborne() {
            ActionInitiationDirective::Reject
        } else {
            ActionInitiationDirective::Allow
        }
    }
}
