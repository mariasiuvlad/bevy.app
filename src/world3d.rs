use crate::animation::CharacterAnimationPlugin;

use crate::input::InputPlugin;
use crate::maps::rogue_world::RogueWorldPlugin;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerTarget;

#[derive(Component)]
pub struct Character(pub CharacterInfo);

pub struct CharacterInfo {
    pub name: String,
}

#[derive(Component)]
pub struct PlayerCamera;

pub struct World3dPlugin;

impl Plugin for World3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RogueWorldPlugin, CharacterAnimationPlugin, InputPlugin));
    }
}
