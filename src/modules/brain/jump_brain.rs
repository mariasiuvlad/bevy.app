use bevy::prelude::*;

use crate::modules::character_controller::{
    actions::JumpAction, CharacterController, WalkMotionType,
};

#[derive(Component)]
pub struct JumpBrain;

pub fn jump_brain_controller(mut brain_query: Query<&mut CharacterController, With<JumpBrain>>) {
    for mut ctr in brain_query.iter_mut() {
        ctr.motion_type(WalkMotionType {
            velocity: Vec3::ZERO,
            ..default()
        });
        ctr.action_type(JumpAction { velocity: Vec3::Y });
    }
}

pub struct JumpBrainPlugin;
impl Plugin for JumpBrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, jump_brain_controller);
    }
}
