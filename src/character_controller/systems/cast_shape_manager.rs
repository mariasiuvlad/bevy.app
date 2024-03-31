use crate::character_controller::CharacterController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn cast_shape_manager(
    mut q: Query<(&mut CharacterController, &Transform, &Collider)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut controller, transform, collider) in q.iter_mut() {
        let shape_vel: Vec3 = -transform.local_y().normalize();
        let mut collider = collider.clone();
        collider.set_scale(Vec3::new(0.9, 1.0, 0.9), 10);

        controller.cast_shape_result = rapier_context.cast_shape(
            transform.translation,
            transform.rotation,
            shape_vel,
            &collider,
            1.5,
            true,
            QueryFilter::only_fixed().exclude_sensors(),
        );
    }
}
