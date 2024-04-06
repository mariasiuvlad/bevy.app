use bevy::prelude::*;
pub struct BrainPlugin;

mod wandering_brain;

pub use wandering_brain::WanderingBrain;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((wandering_brain::WanderingBrainPlugin,));
    }
}
