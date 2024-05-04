use std::time::Duration;

use bevy::prelude::*;

use crate::modules::character_controller::{motion::*, traits::action::*};

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
        ctx: ActionContext,
        _lifecycle: ActionLifecycle,
        motion: &mut Motion,
    ) -> ActionLifecycleDirective {
        match state {
            DashActionState::Started => {
                *state = DashActionState::Active(Timer::from_seconds(0.3, TimerMode::Once));
                motion.linvel += VelChange::impulse(Vec3::from(self.facing) * 10.);
                ActionLifecycleDirective::Active
            }
            DashActionState::Active(timer) => {
                if timer.finished() {
                    *state = DashActionState::Finished;
                } else {
                    timer.tick(Duration::from_secs_f32(ctx.frame_duration));
                }
                ActionLifecycleDirective::Active
            }
            DashActionState::Finished => {
                motion.linvel += VelChange::boost(Vec3::from(self.facing) * -10.);
                ActionLifecycleDirective::Finished
            }
        }
    }

    fn initiation_decision(&self, ctx: ActionContext) -> ActionInitiationDirective {
        if ctx.motion_type.is_airborne() {
            ActionInitiationDirective::Allow
        } else {
            ActionInitiationDirective::Reject
        }
    }
}
