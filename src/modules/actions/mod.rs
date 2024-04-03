use bevy::prelude::*;

use self::jump::JumpPlugin;

pub mod jump;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JumpPlugin);
    }
}
