use bevy::{
    prelude::*,
    utils::{Entry, HashMap},
};
use bevy_rapier3d::{
    dynamics::{ExternalForce, ExternalImpulse, ReadMassProperties, Velocity},
    plugin::RapierConfiguration,
};

use crate::modules::character_controller::traits::action_type::ActionLifecycleDirective;

use self::{
    motion::{apply_motion_system, debug_motion_system, Motion},
    motion_type::{BoxableMotionType, DynamicMotionType, MotionType},
    player_input::player_keyboard_input_system,
    proximity_sensor::{cast_ray_system, ProximitySensor},
    traits::action_type::{
        ActionInitiationDirective, ActionLifecycle, ActionType, ActionTypeContext,
        BoxableActionType, DynamicActionType,
    },
    walk::MotionTypeContext,
};

mod jump;
mod motion;
mod motion_type;
mod player_input;
mod proximity_sensor;
mod traits;
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

struct FedEntry {
    fed_this_frame: bool,
}

#[derive(Default, Component)]
pub struct CharacterController {
    current_motion_type: Option<(&'static str, Box<dyn DynamicMotionType>)>,
    current_action: Option<(&'static str, Box<dyn DynamicActionType>)>,
    contender_action: Option<(&'static str, Box<dyn DynamicActionType>)>,
    actions_being_fed: HashMap<&'static str, FedEntry>,
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

    pub fn action_type<A: ActionType>(&mut self, a: A) {
        self.named_action(A::NAME, a);
    }

    pub fn named_action<A: ActionType>(&mut self, name: &'static str, a: A) {
        match self.actions_being_fed.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().fed_this_frame = true;
                match self.current_action.as_mut() {
                    None => {
                        self.contender_action = Some((name, Box::new(BoxableActionType::new(a))));
                    }
                    Some((current_action_name, current_action)) => {
                        if *current_action_name == name {
                            let current_action = current_action
                                .as_mut_any()
                                .downcast_mut::<BoxableActionType<A>>()
                                .expect("Multiple action types registered with same name {name:?}");

                            current_action.input = a;
                        }
                    }
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(FedEntry {
                    fed_this_frame: true,
                });

                match self.contender_action.as_mut() {
                    Some((_, contender_action)) => {
                        let contender_action = contender_action
                            .as_mut_any()
                            .downcast_mut::<BoxableActionType<A>>()
                            .expect("Multiple action types registered with same name {name:?}");

                        contender_action.input = a;
                    }
                    None => {
                        self.contender_action = Some((name, Box::new(BoxableActionType::new(a))));
                    }
                }
            }
        };
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

            let has_valid_contender = if let Some((_, contender_action)) = &ctr.contender_action {
                let initiation_decision = contender_action.initiation_decision(ActionTypeContext {
                    frame_duration: time.delta_seconds(),
                    gravity: rapier_config.gravity,
                    proximity_sensor_output: sensor.output,
                    transform: *transform,
                    velocity: *velocity,
                    motion_type,
                });

                match initiation_decision {
                    ActionInitiationDirective::Allow => true,
                    ActionInitiationDirective::Reject | ActionInitiationDirective::Delay => {
                        ctr.contender_action = None;
                        false
                    }
                }
            } else {
                false
            };

            match &mut ctr.current_action {
                Some((action_name, action_type)) => {
                    let lifecycle = if ctr
                        .actions_being_fed
                        .get(action_name)
                        .map(|fed_entry| fed_entry.fed_this_frame)
                        .unwrap_or(false)
                    {
                        ActionLifecycle::StillFed
                    } else {
                        ActionLifecycle::NoLongerFed
                    };

                    let directive = action_type.apply(
                        ActionTypeContext {
                            frame_duration: time.delta_seconds(),
                            gravity: rapier_config.gravity,
                            proximity_sensor_output: sensor.output,
                            transform: *transform,
                            velocity: *velocity,
                            motion_type,
                        },
                        lifecycle,
                        motion,
                    );

                    if directive == ActionLifecycleDirective::Finished {
                        ctr.current_action = None
                    }
                }
                None => {
                    if has_valid_contender {
                        let (contender_name, mut contender_action) =
                            ctr.contender_action.take().expect(
                                "has_valid_contender can only be true if contender_action is Some",
                            );
                        contender_action.apply(
                            ActionTypeContext {
                                frame_duration: time.delta_seconds(),
                                gravity: rapier_config.gravity,
                                proximity_sensor_output: sensor.output,
                                transform: *transform,
                                velocity: *velocity,
                                motion_type,
                            },
                            ActionLifecycle::Started,
                            motion,
                        );
                        ctr.current_action = Some((contender_name, contender_action));
                        ctr.contender_action = None;
                    }
                }
            }
        }

        // Cycle actions_being_fed
        ctr.actions_being_fed.retain(|_, fed_entry| {
            if fed_entry.fed_this_frame {
                fed_entry.fed_this_frame = false;
                true
            } else {
                false
            }
        });

        if let Some((contender_name, ..)) = ctr.contender_action {
            if !ctr.actions_being_fed.contains_key(contender_name) {
                ctr.contender_action = None;
            }
        }
    }
}

#[derive(Default, Bundle)]
pub struct CharacterControllerPhysicsBundle {
    velocity: Velocity,
    external_force: ExternalForce,
    impulse: ExternalImpulse,
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
                player_keyboard_input_system,
                cast_ray_system,
            ),
        );
    }
}
