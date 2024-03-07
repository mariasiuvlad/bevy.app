use bevy::prelude::*;

use crate::{
    animated_bundle::{AnimationState, AnimationStates},
    app_state::AppState,
    combat::{combat_stats::Stats, status_effect::sprint::SprintEffect},
    get_single, get_single_mut,
    world3d::{CharacterTarget, Player, PlayerCamera},
};

#[derive(Component)]
pub struct Movement(Vec3);

impl Default for Movement {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

impl Movement {
    pub fn from(x: f32, z: f32) -> Self {
        Self(Vec3::new(x, 0.0, z))
    }
    pub fn x(&self) -> f32 {
        self.0.x
    }
    pub fn z(&self) -> f32 {
        self.0.z
    }
}

fn player_input_to_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut move_q: Query<&mut Movement, With<Player>>,
    camera_q: Query<&Transform, With<PlayerCamera>>,
) {
    let camera_t = get_single!(camera_q);
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

        let mut m: Vec3 = camera_t.forward() * x + camera_t.local_x() * z;
        m.y = 0.0;
        movement.0 = m;
    }
}

fn apply_movement(
    time: Res<Time>,
    mut move_q: Query<
        (&mut Transform, &Movement, &Stats),
        (Without<CharacterTarget>, Without<PlayerCamera>),
    >,
    target_q: Query<&Transform, (With<CharacterTarget>, Without<Player>)>,
) {
    for (mut t, movement, stats) in move_q.iter_mut() {
        let mut look_to: Vec3 = Vec3::new(movement.x(), 0.0, movement.z());

        // movement
        t.translation +=
            look_to.normalize_or_zero() * time.delta_seconds() * stats.computed_move_speed();

        if let Ok(_) = target_q.get_single() {
            return;
        }

        // rotation
        if look_to != Vec3::ZERO && look_to != t.forward().normalize() {
            look_to.y = 0.;

            t.rotation = t.rotation.slerp(
                t.looking_to(look_to.normalize(), Vec3::Y).rotation,
                0.1 * time.delta_seconds() * 80.,
            );
        }
    }
}

fn face_target(
    time: Res<Time>,
    mut targeting_q: Query<&mut Transform, With<Player>>,
    target_q: Query<&Transform, (With<CharacterTarget>, Without<Player>)>,
) {
    let mut t = get_single_mut!(targeting_q);
    let target_transform = get_single!(target_q);

    t.rotation = t.rotation.slerp(
        t.looking_at(target_transform.translation, Vec3::Y).rotation,
        0.1 * time.delta_seconds() * 80.,
    );
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
                face_target,
                movement_animations,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
