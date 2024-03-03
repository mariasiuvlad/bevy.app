use bevy::prelude::*;

use crate::{
    animated_bundle::{AnimationState, AnimationStates},
    app_state::AppState,
    get_single,
    world3d::{Player, PlayerCamera},
};

#[derive(Component)]
pub struct MovementNew {
    pub v: Vec3,
    pub x: f32,
    pub z: f32,
}

impl Default for MovementNew {
    fn default() -> Self {
        Self {
            v: Vec3::ZERO,
            x: 0.,
            z: 0.,
        }
    }
}

impl MovementNew {
    pub fn from(v: Vec3) -> Self {
        Self { v, x: 0., z: 0. }
    }
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut move_q: Query<&mut MovementNew, With<Player>>,
) {
    for mut movement in move_q.iter_mut() {
        let mut x = 0.;
        let mut z = 0.;

        if keys.pressed(KeyCode::KeyW) {
            x += 1.;
        }
        if keys.pressed(KeyCode::KeyA) {
            z -= 1.;
        }
        if keys.pressed(KeyCode::KeyS) {
            x -= 1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            z += 1.;
        }

        movement.x = x;
        movement.z = z;
    }
}

fn apply_movement(
    time: Res<Time>,
    mut move_q: Query<(&MovementNew, &mut Transform), Without<PlayerCamera>>,
    camera_q: Query<&Transform, With<PlayerCamera>>,
) {
    let camera_t = get_single!(camera_q);
    for (movement, mut t) in move_q.iter_mut() {
        let mut v = camera_t.forward() * movement.x + camera_t.local_x() * movement.z;
        v = Vec3::new(v.x, 0., v.z);
        t.translation += v * time.delta_seconds() * 3.;

        if v != Vec3::ZERO {
            let mut look_to = -v.normalize();
            look_to.y = 0.;
            t.look_to(look_to, Vec3::Y);
        }
    }
}

fn movement_animations(mut move_q: Query<(&MovementNew, &mut AnimationState)>) {
    for (movement, mut animation_state) in move_q.iter_mut() {
        if (movement.x != 0. || movement.z != 0.) && animation_state.0 != AnimationStates::Walk {
            *animation_state = AnimationState(AnimationStates::Walk)
        }
        if movement.x == 0. && movement.z == 0. && animation_state.0 != AnimationStates::Idle {
            *animation_state = AnimationState(AnimationStates::Idle)
        }
    }
}

pub struct MovementNewPlugin;

impl Plugin for MovementNewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_movement, player_movement, movement_animations).run_if(in_state(AppState::Game)),
        );
    }
}
