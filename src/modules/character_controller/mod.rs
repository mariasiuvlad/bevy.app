use bevy::{
    ecs::schedule::{InternedScheduleLabel, ScheduleLabel},
    prelude::*,
    utils::{Entry, HashMap},
};
use bevy_rapier3d::{
    dynamics::{ExternalForce, ExternalImpulse, ReadMassProperties, Velocity},
    plugin::RapierConfiguration,
};

use crate::modules::character_controller::traits::action::ActionLifecycleDirective;

use self::{
    motion::{apply_motion_system, debug_motion_system, Motion},
    player_input::player_keyboard_input_system,
    proximity_sensor::{cast_ray_system, ProximitySensor},
    traits::{
        action::{
            Action, ActionContext, ActionInitiationDirective, ActionLifecycle, BoxableActionType,
            DynamicActionType,
        },
        basis::{Basis, BasisContext, BoxableBasis, DynamicBasis},
    },
};

mod dash;
mod jump;
mod motion;
mod player_input;
mod proximity_sensor;
mod traits;
mod utils;
mod walk;

pub use walk::WalkMotionType;

/// The user controls should be applied in this system set.
#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct UserControlsSystemSet;

/// Umbrella system set for [`TnuaPipelineStages`].
///
/// The physics backends' plugins are responsible for preventing this entire system set from
/// running when the physics backend itself is paused.
#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct CharacterControllerSystemSet;

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub enum TnuaPipelineStages {
    /// Data is read from the physics backend.
    Sensors,
    /// Tnua decieds how the entity should be manipulated.
    Logic,
    /// Forces are applied in the physics backend.
    Motors,
}

#[derive(Default, Bundle)]
pub struct CharacterControllerBundle {
    controller: CharacterController,
    physics: CharacterControllerPhysicsBundle,
    motion: Motion,
    proximity_sensor: ProximitySensor,
}

#[derive(Debug)]
struct FedEntry {
    fed_this_frame: bool,
}

#[derive(Default, Component)]
pub struct CharacterController {
    current_basis: Option<(&'static str, Box<dyn DynamicBasis>)>,
    current_action: Option<(&'static str, Box<dyn DynamicActionType>)>,
    contender_action: Option<(&'static str, Box<dyn DynamicActionType>)>,
    actions_being_fed: HashMap<&'static str, FedEntry>,
}

impl CharacterController {
    pub fn motion_type<M: Basis>(&mut self, m: M) {
        self.named_motion_type(M::NAME, m);
    }

    pub fn named_motion_type<M: Basis>(&mut self, name: &'static str, motion_type: M) -> &mut Self {
        if let Some((existing_name, existing_motion_type)) =
            self.current_basis.as_mut().and_then(|(n, m)| {
                let m = m.as_mut_any().downcast_mut::<BoxableBasis<M>>()?;
                Some((n, m))
            })
        {
            *existing_name = name;
            existing_motion_type.input = motion_type;
        } else {
            self.current_basis = Some((name, Box::new(BoxableBasis::new(motion_type))))
        }
        self
    }

    pub fn action_type<A: Action>(&mut self, a: A) {
        self.named_action(A::NAME, a);
    }

    pub fn named_action<A: Action>(&mut self, name: &'static str, a: A) {
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
                                .expect("[Occupied] Multiple action types registered with same name {name:?}");

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
                            .expect(
                                "[Vacant] Multiple action types registered with same name {name:?}",
                            );

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

        if let Some((_, motion_type)) = &mut ctr.current_basis {
            let motion_type = motion_type.as_mut();
            motion_type.apply(
                BasisContext {
                    frame_duration: time.delta_seconds(),
                    proximity_sensor_output: sensor.output,
                    transform: *transform,
                    velocity: *velocity,
                    gravity: rapier_config.gravity,
                },
                motion,
            );

            let has_valid_contender = if let Some((_, contender_action)) = &ctr.contender_action {
                let initiation_decision = contender_action.initiation_decision(ActionContext {
                    frame_duration: time.delta_seconds(),
                    gravity: rapier_config.gravity,
                    proximity_sensor_output: sensor.output,
                    transform: *transform,
                    velocity: *velocity,
                    motion_type,
                });

                match initiation_decision {
                    ActionInitiationDirective::Allow => true,
                    ActionInitiationDirective::Reject => {
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
                        ActionContext {
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

                    // info!("current_action {action_name:?}, lifecycle  {lifecycle:?}, directive {directive:?}");

                    match directive {
                        ActionLifecycleDirective::Active => {}
                        ActionLifecycleDirective::Finished => {
                            // info!(
                            //     "current_action {} finished, has_valid_contender {}",
                            //     action_name, has_valid_contender,
                            // );
                            ctr.current_action = if has_valid_contender {
                                let (contender_name, mut contender_action) =
                                    ctr.contender_action.take().expect(
                                        "has_valid_contender can only be true if contender_action is Some",
                                    );
                                contender_action.apply(
                                    ActionContext {
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
                                // ctr.contender_action = None;
                                Some((contender_name, contender_action))
                            } else {
                                None
                            }
                        }
                    }
                }
                None => {
                    if has_valid_contender {
                        let (contender_name, mut contender_action) =
                            ctr.contender_action.take().expect(
                                "has_valid_contender can only be true if contender_action is Some",
                            );
                        contender_action.apply(
                            ActionContext {
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

pub struct CharacterControllerPlugin {
    schedule: InternedScheduleLabel,
}

impl CharacterControllerPlugin {
    pub fn new(schedule: impl ScheduleLabel) -> Self {
        Self {
            schedule: schedule.intern(),
        }
    }
}

impl Default for CharacterControllerPlugin {
    fn default() -> Self {
        Self::new(Update)
    }
}

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            self.schedule,
            (
                TnuaPipelineStages::Sensors,
                UserControlsSystemSet,
                TnuaPipelineStages::Logic,
                TnuaPipelineStages::Motors,
            )
                .chain()
                .in_set(CharacterControllerSystemSet),
        );

        app.add_systems(
            self.schedule,
            cast_ray_system.in_set(TnuaPipelineStages::Sensors),
        );

        app.add_systems(
            self.schedule,
            player_keyboard_input_system.in_set(UserControlsSystemSet),
        );

        app.add_systems(
            self.schedule,
            controller_system.in_set(TnuaPipelineStages::Logic),
        );

        app.add_systems(
            self.schedule,
            (apply_motion_system, debug_motion_system).in_set(TnuaPipelineStages::Motors),
        );
    }
}
