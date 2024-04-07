use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::{ExternalForce, ReadMassProperties, Velocity},
    plugin::RapierConfiguration,
};

use crate::world3d::{Player, PlayerCamera};

use self::{
    jump::{ActionType, ActionTypeContext, JumpActionType},
    motion::{apply_motion_system, debug_motion_system, Motion},
    motion_type::{BoxableMotionType, DynamicMotionType, MotionType},
    proximity_sensor::{cast_ray_system, ProximitySensor},
    walk::MotionTypeContext,
};

mod jump;
mod motion;
mod motion_type;
mod proximity_sensor;
mod utils;
mod walk;

pub use walk::WalkMotionType;

#[derive(Default, Bundle)]
pub struct CharacterControllerBundle {
    controller: CharacterController,
    physics: CharacterControllerPhysicsBundle,
    motion: Motion,
    proximity_sensor: ProximitySensor,
}

#[derive(Default, Component)]
pub struct CharacterController {
    current_motion_type: Option<(&'static str, Box<dyn DynamicMotionType>)>,
    current_action: Option<JumpActionType>,
}

impl CharacterController {
    pub fn motion_type<M: MotionType>(&mut self, m: M) {
        self.named_motion_type(M::NAME, m);
    }

    pub fn named_motion_type<M: MotionType>(
        &mut self,
        name: &'static str,
        motion_type: M,
    ) -> &mut Self {
        if let Some((existing_name, existing_motion_type)) =
            self.current_motion_type.as_mut().and_then(|(n, m)| {
                let m = m.as_mut_any().downcast_mut::<BoxableMotionType<M>>()?;
                Some((n, m))
            })
        {
            *existing_name = name;
            existing_motion_type.input = motion_type;
        } else {
            self.current_motion_type = Some((name, Box::new(BoxableMotionType::new(motion_type))))
        }
        self
    }

    pub fn action_type(&mut self, a: Option<JumpActionType>) {
        self.current_action = a;
    }
}

pub fn keyboard_input_system(
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
            velocity: velocity.normalize_or_zero() * 10.,
            facing,
            ..default()
        });

        if keyboard.pressed(KeyCode::Space) {
            ctr.action_type(Some(JumpActionType { velocity: Vec3::Y }));
        }

        if keyboard.just_released(KeyCode::Space) {
            ctr.action_type(None);
        }
    }
}

pub fn controller_system(
    time: Res<Time>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(
        &Transform,
        &Velocity,
        &mut CharacterController,
        &ProximitySensor,
        &mut motion::Motion,
    )>,
) {
    rapier_config.gravity;
    for (transform, velocity, mut ctr, sensor, mut motion) in query.iter_mut() {
        let ctr = ctr.as_mut();
        let motion = motion.as_mut();

        if let Some((_, motion_type)) = &mut ctr.current_motion_type {
            let motion_type = motion_type.as_mut();
            motion_type.apply(
                MotionTypeContext {
                    frame_duration: time.delta_seconds(),
                    proximity_sensor_output: sensor.output,
                    transform: *transform,
                    velocity: *velocity,
                    gravity: rapier_config.gravity,
                },
                motion,
            );

            if let Some(action_type) = &mut ctr.current_action {
                action_type.apply(
                    ActionTypeContext {
                        frame_duration: time.delta_seconds(),
                        gravity: rapier_config.gravity,
                        proximity_sensor_output: sensor.output,
                        transform: *transform,
                        velocity: *velocity,
                        motion_type,
                    },
                    motion,
                );
            }
        }
    }
}

#[derive(Default, Bundle)]
pub struct CharacterControllerPhysicsBundle {
    velocity: Velocity,
    external_force: ExternalForce,
    read_mass_properties: ReadMassProperties,
}

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_motion_system,
                debug_motion_system,
                controller_system,
                keyboard_input_system,
                cast_ray_system,
            ),
        );
    }
}
