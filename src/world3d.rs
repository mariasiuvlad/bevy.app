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
