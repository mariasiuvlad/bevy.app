use bevy::prelude::*;

pub struct ProjectionPlaneForRotation {
    pub forward: Vec3,
    pub sideways: Vec3,
}

impl ProjectionPlaneForRotation {
    pub fn from_up_and_fowrard(up: Direction3d, forward: Vec3) -> Self {
        Self {
            forward,
            sideways: up.cross(forward),
        }
    }

    pub fn from_up_using_default_forward(up: Direction3d) -> Self {
        Self::from_up_and_fowrard(up, Vec3::NEG_Z)
    }

    pub fn project_and_normalize(&self, vector: Vec3) -> Vec2 {
        Vec2::new(vector.dot(self.forward), vector.dot(self.sideways)).normalize_or_zero()
    }

    pub fn rotation_to_set_forward(&self, current_forward: Vec3, desired_forward: Vec3) -> f32 {
        let rotation_to_set_forward = Quat::from_rotation_arc_2d(
            self.project_and_normalize(current_forward),
            self.project_and_normalize(desired_forward),
        );
        rotation_to_set_forward.xyz().z
    }
}
