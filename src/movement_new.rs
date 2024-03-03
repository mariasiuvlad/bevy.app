use bevy::prelude::*;

use crate::{
    animated_bundle::{AnimationState, AnimationStates},
    app_state::AppState,
    get_single,
    world3d::{Player, PlayerCamera},
};

#[derive(Component)]
pub struct MovementNew {
    pub x: f32,
    pub z: f32,
}

impl Default for MovementNew {
    fn default() -> Self {
        Self { x: 0., z: 0. }
    }
}

impl MovementNew {
    pub fn from(x: f32, z: f32) -> Self {
        Self { x, z }
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
        let mut look_to: Vec3 = camera_t.forward() * movement.x + camera_t.local_x() * movement.z;
        look_to.y = 0.;

        // movement
        t.translation += look_to.normalize_or_zero() * time.delta_seconds() * 3.;

        // rotation
        if look_to != Vec3::ZERO && look_to != t.forward().normalize() {
            look_to.y = 0.;

            t.rotation = t.rotation.slerp(
                t.looking_to(-look_to.normalize(), Vec3::Y).rotation,
                0.1 * time.delta_seconds() * 80.,
            );
        }
    }
}

fn apply_rotation(
    time: Res<Time>,
    mut move_q: Query<(&MovementNew, &mut Transform), Without<PlayerCamera>>,
    camera_q: Query<&Transform, With<PlayerCamera>>,
) {
    let camera_t = get_single!(camera_q);
    for (movement, mut t) in move_q.iter_mut() {
        let mut look_to = camera_t.forward() * movement.x + camera_t.local_x() * movement.z;
        look_to.y = 0.;

        if look_to != Vec3::ZERO && look_to != t.forward().normalize() {
            look_to.y = 0.;

            t.rotation = t.rotation.slerp(
                t.looking_to(-look_to, Vec3::Y).rotation,
                0.1 * time.delta_seconds() * 80.,
            );
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
            (
                player_movement,
                (apply_movement).after(player_movement),
                movement_animations,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
