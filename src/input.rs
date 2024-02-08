use bevy::prelude::*;

use crate::{
    app_state::AppState,
    combat::AttackEvent,
    world3d::{Character, Player, PlayerTarget},
};

const SPEED: f32 = 4.0;
const TURN_RATE: f32 = 5.0;

#[derive(Event)]
struct TargetNextEnemyEvent;

fn handle_target_next_enemy(
    mut commands: Commands,
    mut ev_target_next_enemy: EventReader<TargetNextEnemyEvent>,
    character_query: Query<(Entity, &Character), (Without<PlayerTarget>, Without<Player>)>,
    character_query_target: Query<(Entity, &Character), With<PlayerTarget>>,
) {
    for _ in ev_target_next_enemy.read() {
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

// @TODO remove *character_query* maybe use events
fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ev_target_next_enemy: EventWriter<TargetNextEnemyEvent>,
    mut ev_attack: EventWriter<AttackEvent>,
    mut player_query: Query<&mut Transform, With<Player>>,
    player_target_query: Query<Entity, With<PlayerTarget>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        todo!("Jump!")
    }
    if keys.just_pressed(KeyCode::Tab) {
        ev_target_next_enemy.send(TargetNextEnemyEvent);
    }
    if keys.just_pressed(KeyCode::R) {
        if let Ok(player_target_handle) = player_target_query.get_single() {
            info!("Attack!");
            ev_attack.send(AttackEvent::new(
                player_target_handle,
                player_target_handle,
                5,
            ));
        }
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
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TargetNextEnemyEvent>().add_systems(
            Update,
            (keyboard_input, handle_target_next_enemy).run_if(in_state(AppState::Game)),
        );
    }
}
