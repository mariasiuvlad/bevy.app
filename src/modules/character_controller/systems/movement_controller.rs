use super::super::CharacterController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const MOVE_SPEED: f32 = 8.;
const WALK_SPEED: f32 = 1.;

pub fn movement_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut ctr_q: Query<(&CharacterController, &mut Velocity)>,
) {
    let speed = match keys.pressed(KeyCode::KeyC) {
        false => MOVE_SPEED,
        true => WALK_SPEED,
    };

    for (ctr, mut velocity) in ctr_q.iter_mut() {
        let normalized_movement = ctr.transform.translation.normalize_or_zero();

        let final_movement = ctr.transform.forward() * normalized_movement.x
            + ctr.transform.local_x() * normalized_movement.z;

        if ctr.is_grounded() {
            velocity.linvel.x = final_movement.x * speed;
            velocity.linvel.z = final_movement.z * speed;
        }
    }
}
