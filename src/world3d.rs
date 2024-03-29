use crate::input::InputPlugin;
use crate::maps::physics_platformer::PhysicsPlatformerPlugin;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterTarget;

#[derive(Component)]
pub struct Target(Entity);

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
        app.add_plugins((PhysicsPlatformerPlugin, InputPlugin));
    }
}
