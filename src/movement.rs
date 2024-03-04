use bevy::prelude::*;

use crate::{
    animated_bundle::{AnimationState, AnimationStates},
    app_state::AppState,
    combat::{combat_stats::Stats, status_effect::sprint::SprintEffect},
    get_single,
    world3d::{Player, PlayerCamera},
};

#[derive(Component)]
pub struct Movement(Vec2);

impl Default for Movement {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

impl Movement {
    pub fn from(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
    pub fn x(&self) -> f32 {
        self.0.x
    }
    pub fn z(&self) -> f32 {
        self.0.y
    }
}

fn player_input_to_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut move_q: Query<&mut Movement, With<Player>>,
) {
    for mut movement in move_q.iter_mut() {
        let mut v = Vec2::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            v.x += 1.;
        }
        if keys.pressed(KeyCode::KeyA) {
            v.y -= 1.;
        }
        if keys.pressed(KeyCode::KeyS) {
            v.x -= 1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            v.y += 1.;
        }

        movement.0 = v.normalize_or_zero();
    }
}

fn apply_movement(
    time: Res<Time>,
    mut move_q: Query<(&mut Transform, &Movement, &Stats), Without<PlayerCamera>>,
    camera_q: Query<&Transform, With<PlayerCamera>>,
) {
    let camera_t = get_single!(camera_q);
    for (mut t, movement, stats) in move_q.iter_mut() {
        let mut look_to: Vec3 =
            camera_t.forward() * movement.x() + camera_t.local_x() * movement.z();
        look_to.y = 0.;

        // movement
        t.translation +=
            look_to.normalize_or_zero() * time.delta_seconds() * stats.computed_move_speed();

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

fn movement_animations(mut move_q: Query<(&Movement, Option<&SprintEffect>, &mut AnimationState)>) {
    for (movement, sprint_effect, mut animation_state) in move_q.iter_mut() {
        let movement_animation = match sprint_effect {
            Some(_) => AnimationStates::Run,
            None => AnimationStates::Walk,
        };
        if (movement.x() != 0. || movement.z() != 0.) && animation_state.0 != movement_animation {
            *animation_state = AnimationState(movement_animation)
        }
        if movement.x() == 0. && movement.z() == 0. && animation_state.0 == movement_animation {
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
                player_input_to_movement,
                (apply_movement).after(player_input_to_movement),
                movement_animations,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
