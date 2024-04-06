use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use bevy_rapier3d::dynamics::{ExternalForce, ReadMassProperties, Velocity};

#[derive(Default, Debug, Clone)]
pub struct VelChange {
    pub accel: Vec3,
    pub boost: Vec3,
}

impl VelChange {
    pub fn boost(boost: Vec3) -> Self {
        Self {
            accel: default(),
            boost,
        }
    }

    pub fn accel(accel: Vec3) -> Self {
        Self {
            accel,
            boost: default(),
        }
    }
}

impl Add<VelChange> for VelChange {
    type Output = VelChange;

    fn add(self, rhs: VelChange) -> Self::Output {
        Self::Output {
            accel: self.accel + rhs.accel,
            boost: self.boost + rhs.boost,
        }
    }
}

impl AddAssign for VelChange {
    fn add_assign(&mut self, rhs: Self) {
        self.accel += rhs.accel;
        self.boost += rhs.boost;
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
        &ReadMassProperties,
    )>,
) {
    for (motion, mut force, mut velocity, _mass_properties) in query.iter_mut() {
        velocity.linvel += motion.linvel.boost;
        velocity.angvel += motion.angvel.boost;

        force.force = motion.linvel.accel * _mass_properties.get().mass;
        force.torque = motion.angvel.accel * _mass_properties.get().mass;
    }
}
