use bevy::prelude::*;

use crate::world3d::{Player, PlayerCamera};

use super::{jump::JumpActionType, CharacterController, WalkMotionType};

pub fn player_keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ctr_query: Query<&mut CharacterController, With<Player>>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
    if let Ok(mut ctr) = ctr_query.get_single_mut() {
        let mut velocity = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            velocity += Vec3::NEG_Z;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            velocity += Vec3::Z;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            velocity += Vec3::NEG_X;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            velocity += Vec3::X;
        }

        let (facing, velocity) = match camera_query.get_single() {
            Ok(transform) => {
                velocity = transform.local_z() * velocity.z + transform.local_x() * velocity.x;
                match velocity != Vec3::ZERO {
                    true => (
                        Some(Direction3d::new_unchecked(velocity.normalize_or_zero())),
                        velocity,
                    ),
                    false => (None, velocity),
                }
            }
            Err(_) => (None, velocity),
        };

        ctr.motion_type(WalkMotionType {
            velocity: velocity.normalize_or_zero() * 15.,
            facing,
            ..default()
        });

        if keyboard.pressed(KeyCode::Space) {
            ctr.action_type(JumpActionType { velocity: Vec3::Y });
        }
    }
}
