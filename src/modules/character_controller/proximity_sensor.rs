use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Component)]
pub struct ProximitySensor {
    pub origin: Vec3,
    pub direction: Vec3,
    pub output: Option<ProximitySensorOutput>,
}

impl Default for ProximitySensor {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            direction: Vec3::NEG_Y,
            output: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProximitySensorOutput {
    pub entity: Entity,
    pub distance: f32,
}

pub fn _cast_shape_system(
    mut q: Query<(&mut ProximitySensor, &Transform, &Collider)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut sensor, transform, collider) in q.iter_mut() {
        let cast_shape_result = rapier_context.cast_shape(
            transform.translation + sensor.origin,
            transform.rotation,
            sensor.direction,
            &collider,
            1.5,
            true,
            QueryFilter::only_fixed().exclude_sensors(),
        );

        sensor.output = match cast_shape_result {
            None => None,
            Some((entity, toi)) => Some(ProximitySensorOutput {
                entity,
                distance: toi.toi,
            }),
        }
    }
}

pub fn cast_ray_system(
    mut q: Query<(&mut ProximitySensor, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut sensor, transform) in q.iter_mut() {
        let result = rapier_context.cast_ray(
            transform.translation + sensor.origin,
            sensor.direction,
            3.,
            false,
            QueryFilter::only_fixed().exclude_sensors(),
        );

        sensor.output = match result {
            None => None,
            Some((entity, distance)) => Some(ProximitySensorOutput { entity, distance }),
        }
    }
}
