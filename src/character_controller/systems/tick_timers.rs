use super::super::CharacterController;
use bevy::prelude::*;

pub fn tick_timers(time: Res<Time>, mut q: Query<&mut CharacterController>) {
    for mut ctr in q.iter_mut() {
        ctr.jump_timer.tick(time.delta());
    }
}
