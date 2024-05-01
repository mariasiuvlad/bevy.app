use std::time::Duration;

use bevy::prelude::*;

use super::{
    motion,
    traits::action::{self, Action},
};

#[derive(Default, Debug)]
pub enum DashActionState {
    #[default]
    Started,
    Active(Timer),
    Finished,
}

#[derive(Clone, Copy)]
pub struct DashAction {
    pub facing: Direction3d,
}

impl Action for DashAction {
    const NAME: &'static str = "Dash";

    type State = DashActionState;

    fn apply(
        &self,
        state: &mut Self::State,
        ctx: action::ActionContext,
        _lifecycle: action::ActionLifecycle,
        motion: &mut motion::Motion,
    ) -> action::ActionLifecycleDirective {
        match state {
            DashActionState::Started => {
                *state = DashActionState::Active(Timer::from_seconds(0.3, TimerMode::Once));
                motion.linvel += motion::VelChange::impulse(Vec3::from(self.facing) * 10.);
                action::ActionLifecycleDirective::Active
            }
            DashActionState::Active(timer) => {
                if timer.finished() {
                    *state = DashActionState::Finished;
                } else {
                    timer.tick(Duration::from_secs_f32(ctx.frame_duration));
                }
                action::ActionLifecycleDirective::Active
            }
            DashActionState::Finished => {
                motion.linvel += motion::VelChange::boost(Vec3::from(self.facing) * -10.);
                action::ActionLifecycleDirective::Finished
            }
        }
    }

    fn initiation_decision(&self, ctx: action::ActionContext) -> action::ActionInitiationDirective {
        if ctx.motion_type.is_airborne() {
            action::ActionInitiationDirective::Allow
        } else {
            action::ActionInitiationDirective::Allow
        }
    }
}
