use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::pipeline::QueryFilter;

use crate::modules::character_controller::{motion::Motion, traits::action::*};

#[derive(Default, Debug)]
pub enum AttackActionState {
    #[default]
    Started,
    WindUp(Timer),
    Backswing(Timer),
    Finished,
}

pub struct AttackAction;

impl Action for AttackAction {
    const NAME: &'static str = "Attack";

    type State = AttackActionState;

    fn apply(
        &self,
        state: &mut Self::State,
        ctx: ActionContext,
        _lifecycle: ActionLifecycle,
        _motion: &mut Motion,
    ) -> ActionLifecycleDirective {
        match state {
            AttackActionState::Started => {
                *state = AttackActionState::WindUp(Timer::from_seconds(0.3, TimerMode::Once));
                ActionLifecycleDirective::Active
            }
            AttackActionState::WindUp(timer) => {
                let facing = ctx.transform.forward().normalize();
                if timer.finished() {
                    // @todo attack logic
                    let res = ctx.rapier_context.cast_ray(
                        ctx.transform.translation + facing,
                        ctx.transform.forward().normalize(),
                        3.,
                        false,
                        QueryFilter::default(),
                    );

                    match res {
                        None => {
                            info!("It hit nothing!");
                        }
                        Some((e, distance)) => {
                            info!("Hit on {:?} at distance {}", e, distance);
                        }
                    }

                    *state =
                        AttackActionState::Backswing(Timer::from_seconds(0.3, TimerMode::Once));
                } else {
                    timer.tick(Duration::from_secs_f32(ctx.frame_duration));
                }

                ActionLifecycleDirective::Active
            }
            AttackActionState::Backswing(timer) => {
                if timer.finished() {
                    *state = AttackActionState::Finished;
                } else {
                    timer.tick(Duration::from_secs_f32(ctx.frame_duration));
                }
                ActionLifecycleDirective::Active
            }
            AttackActionState::Finished => ActionLifecycleDirective::Finished,
        }
    }

    fn initiation_decision(&self, _ctx: ActionContext) -> ActionInitiationDirective {
        ActionInitiationDirective::Allow
    }
}
