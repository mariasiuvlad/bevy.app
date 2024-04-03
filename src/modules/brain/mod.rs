use bevy::prelude::*;
pub struct BrainPlugin;

mod jump_brain;

pub use jump_brain::JumpBrain;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(jump_brain::JumpBrainPlugin);
    }
}
