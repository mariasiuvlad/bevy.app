use super::super::CharacterController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn floating_controller(mut q: Query<(&CharacterController, &mut ExternalForce, &Velocity)>) {
    for (controller, mut force, velocity) in q.iter_mut() {
        let spring_force = match controller.cast_shape_result {
            None => 0.,
            Some((_, toi)) => match controller.is_grounded() {
                false => 0.,
                true => controller.get_computed_spring_force(toi.toi, velocity.linvel.y),
            },
        };

        force.force.y = spring_force;
    }
}
