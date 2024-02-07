use bevy::prelude::*;

use crate::world3d::{Character, Player, PlayerTarget};

const SPEED: f32 = 4.0;
const TURN_RATE: f32 = 5.0;

// @TODO remove *character_query* maybe use events
pub fn keyboard_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    character_query: Query<(Entity, &Character), (Without<PlayerTarget>, Without<Player>)>,
    character_query_target: Query<(Entity, &Character), With<PlayerTarget>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        todo!("Jump!")
    }
    if keys.pressed(KeyCode::W) {
        for mut transform in player_query.iter_mut() {
            let delta = transform.forward() * SPEED * time.delta_seconds();
            transform.translation += delta;
        }
    }
    if keys.pressed(KeyCode::A) {
        for mut transform in player_query.iter_mut() {
            transform.rotate_y(time.delta_seconds() * TURN_RATE);
        }
    }
    if keys.pressed(KeyCode::S) {
        for mut transform in player_query.iter_mut() {
            let delta = transform.forward() * SPEED * time.delta_seconds();
            transform.translation -= delta;
        }
    }
    if keys.pressed(KeyCode::D) {
        for mut transform in player_query.iter_mut() {
            transform.rotate_y(-time.delta_seconds() * TURN_RATE);
        }
    }
    if keys.just_pressed(KeyCode::Tab) {
        let mut new_target: Option<Entity> = None;
        for (handle, _character) in character_query.iter() {
            new_target = Some(handle);
        }

        for (e, _c) in character_query_target.iter() {
            match commands.get_entity(e) {
                Some(mut ec) => {
                    ec.remove::<PlayerTarget>();
                }
                None => {}
            }
        }

        match new_target {
            Some(target) => match commands.get_entity(target) {
                Some(mut entity) => {
                    entity.insert(PlayerTarget);
                }
                None => {}
            },
            None => {}
        }
    }
}
