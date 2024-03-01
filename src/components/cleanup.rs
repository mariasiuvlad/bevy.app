use bevy::prelude::*;

#[derive(Component)]
pub struct LevelUnload;

#[derive(Component)]
pub struct MenuClose;

pub fn cleanup<T: Component>(mut commands: Commands, mut query: Query<Entity, With<T>>) {
    for e in query.iter_mut() {
        commands.entity(e).despawn_recursive()
    }
}
