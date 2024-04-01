use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerTarget;

#[derive(Component)]
pub struct Target(Entity);

#[derive(Component)]
pub struct Targetable;

#[derive(Component)]
pub struct PlayerCamera;
