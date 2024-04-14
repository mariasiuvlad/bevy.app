use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_rapier3d::dynamics::{ExternalForce, ExternalImpulse, ReadMassProperties, Velocity};

use crate::world3d::Player;

#[derive(Default, Debug, Clone)]
pub struct VelChange {
    pub accel: Vec3,
    pub boost: Vec3,
    pub impulse: Vec3,
}

impl VelChange {
    pub const ZERO: Self = Self {
        accel: Vec3::ZERO,
        boost: Vec3::ZERO,
        impulse: Vec3::ZERO,
    };

    pub fn boost(boost: Vec3) -> Self {
        Self { boost, ..default() }
    }

    pub fn accel(accel: Vec3) -> Self {
        Self { accel, ..default() }
    }

    pub fn impulse(impulse: Vec3) -> Self {
        Self {
            impulse,
            ..default()
        }
    }
}

impl Add<VelChange> for VelChange {
    type Output = VelChange;

    fn add(self, rhs: VelChange) -> Self::Output {
        Self::Output {
            accel: self.accel + rhs.accel,
            boost: self.boost + rhs.boost,
            impulse: self.impulse + rhs.impulse,
        }
    }
}

impl AddAssign for VelChange {
    fn add_assign(&mut self, rhs: Self) {
        self.accel += rhs.accel;
        self.boost += rhs.boost;
        self.impulse += rhs.impulse;
    }
}

#[derive(Debug, Default, Component)]
pub struct Motion {
    pub linvel: VelChange,
    pub angvel: VelChange,
}

pub fn apply_motion_system(
    mut query: Query<(
        &Motion,
        &mut ExternalForce,
        &mut Velocity,
        &mut ExternalImpulse,
        &ReadMassProperties,
    )>,
) {
    for (motion, mut force, mut velocity, mut impulse, _mass_properties) in query.iter_mut() {
        let mass = _mass_properties.get().mass;
        velocity.linvel += motion.linvel.boost;
        velocity.angvel += motion.angvel.boost;

        force.force = motion.linvel.accel * mass;
        force.torque = motion.angvel.accel * mass;

        impulse.impulse = motion.linvel.impulse * mass;
        impulse.torque_impulse = motion.angvel.impulse * mass;
    }
}

pub fn debug_motion_system(
    mut contexts: EguiContexts,
    query: Query<
        (
            &Motion,
            &ExternalForce,
            &Velocity,
            &ExternalImpulse,
            &ReadMassProperties,
        ),
        With<Player>,
    >,
) {
    for (motion, force, velocity, impulse, _mass_properties) in query.iter() {
        egui::Window::new("Player motion").show(contexts.ctx_mut(), |ui| {
            ui.label(format!("motion::linvel {:?}", motion.linvel));
            ui.label(format!("motion::angvel {:?}", motion.angvel));

            ui.label(format!(
                "linear: velocity {:?} / force {:?}",
                velocity.linvel.ceil(),
                force.force.ceil()
            ));
            ui.label(format!(
                "angular: velocity {:?} / force {:?}",
                velocity.angvel.ceil(),
                force.torque.ceil()
            ));

            ui.label(format!("{:?}", impulse));
        });
    }
}
