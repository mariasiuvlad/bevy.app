use super::super::CharacterController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn cast_shape_manager(
    mut q: Query<(&mut CharacterController, &Transform, &Collider)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut controller, transform, collider) in q.iter_mut() {
        controller.cast_shape_result = rapier_context.cast_shape(
            transform.translation,
            transform.rotation,
            Vec3::NEG_Y,
            &collider,
            1.5,
            true,
            QueryFilter::only_fixed().exclude_sensors(),
        );
    }
}
