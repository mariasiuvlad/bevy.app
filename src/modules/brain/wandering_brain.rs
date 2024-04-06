use bevy::prelude::*;

use crate::modules::character_controller::{CharacterController, WalkMotionType};

#[derive(Component)]
pub struct WanderingBrain;

pub fn wandering_brain_controller(
    mut brain_query: Query<&mut CharacterController, With<WanderingBrain>>,
) {
    for mut ctr in brain_query.iter_mut() {
        ctr.motion_type(WalkMotionType {
            velocity: Vec3::NEG_Z * 10.,
            ..default()
        })
    }
}

pub struct WanderingBrainPlugin;
impl Plugin for WanderingBrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wandering_brain_controller);
    }
}
