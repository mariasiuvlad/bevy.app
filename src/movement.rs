use bevy::prelude::*;

use crate::{app_state::AppState, combat::combat_stats::Stats};

const TURN_RATE: f32 = 2.0;

pub enum WalkDirection {
    Forward,
    Backward,
}

pub enum StrafeDirection {
    Left,
    Right,
}

pub enum TurnDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct Strafing(pub StrafeDirection);

#[derive(Component)]
pub struct Walking(pub WalkDirection);

#[derive(Component)]
pub struct Turning(pub TurnDirection);

#[derive(Component)]
pub struct Running;

pub struct MovementPlugin;

pub fn handle_walking(
    time: Res<Time>,
    mut walking_query: Query<(&mut Transform, &Stats, &Walking), With<Walking>>,
) {
    for (mut transform, stats, walking) in walking_query.iter_mut() {
        let delta = transform.forward() * stats.computed_move_speed() * time.delta_seconds();

        transform.translation += match walking.0 {
            WalkDirection::Forward => delta,
            WalkDirection::Backward => -delta,
        }
    }
}

pub fn handle_strafing(
    time: Res<Time>,
    mut strafing_query: Query<(&mut Transform, &Stats, &Strafing)>,
) {
    for (mut transform, stats, strafing) in strafing_query.iter_mut() {
        let delta = transform.left() * stats.computed_move_speed() * time.delta_seconds();

        transform.translation += match strafing.0 {
            StrafeDirection::Left => delta,
            StrafeDirection::Right => -delta,
        }
    }
}

pub fn handle_turning(time: Res<Time>, mut turning_query: Query<(&mut Transform, &Turning)>) {
    for (mut transform, strafing) in turning_query.iter_mut() {
        let delta = time.delta_seconds() * TURN_RATE;
        let rotation = match strafing.0 {
            TurnDirection::Left => delta,
            TurnDirection::Right => -delta,
        };

        transform.rotate_y(rotation);
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_walking, handle_strafing, handle_turning).run_if(in_state(AppState::Game)),
        );
    }
}
