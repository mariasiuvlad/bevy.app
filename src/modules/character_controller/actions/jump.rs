use std::time::Duration;

use bevy::prelude::*;

use crate::modules::character_controller::{motion::*, traits::action::*, WalkMotionType};

#[derive(Default, Debug)]
pub enum JumpActionState {
    #[default]
    Started,
    Active(Timer),
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
    const NAME: &'static str = "Jump";

    fn apply(
        &self,
        state: &mut Self::State,
        ctx: ActionContext,
        _lifecycle: ActionLifecycle,
        motion: &mut Motion,
    ) -> ActionLifecycleDirective {
        let current_motion_type = ctx.concrete_motion_type::<WalkMotionType>();
        if let Some((_, motion_state)) = current_motion_type {
            let boost = (Vec3::Y * -motion_state.spring_force).max(Vec3::ZERO);
            motion.linvel += VelChange::boost(boost);
        }

        match state {
            JumpActionState::Started => {
                motion.linvel += VelChange::impulse(Vec3::Y * 5.);
                *state = JumpActionState::Active(Timer::from_seconds(0.75, TimerMode::Once));
                ActionLifecycleDirective::Active
            }
            JumpActionState::Active(timer) => {
                if timer.finished() {
                    *state = JumpActionState::Finished;
                } else {
                    timer.tick(Duration::from_secs_f32(ctx.frame_duration));
                }
                ActionLifecycleDirective::Active
            }
            JumpActionState::Finished => ActionLifecycleDirective::Finished,
        }
    }

    type State = JumpActionState;

    fn initiation_decision(&self, ctx: ActionContext) -> ActionInitiationDirective {
        if ctx.motion_type.is_airborne() {
            ActionInitiationDirective::Reject
        } else {
            ActionInitiationDirective::Allow
        }
    }
}
