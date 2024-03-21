use std::time::Duration;

use bevy::prelude::*;

use crate::{
    app_state::AppState,
    combat::{
        attack::{AttackCooldown, AttackEvent, AttackWindUp},
        status_effect::sprint::SprintEffect,
    },
    get_single,
    world3d::{Character, CharacterTarget, Player},
};

#[derive(Event)]
struct TargetNextEnemyEvent;

fn handle_target_next_enemy(
    mut commands: Commands,
    mut ev_target_next_enemy: EventReader<TargetNextEnemyEvent>,
    character_query: Query<(Entity, &Character), (Without<CharacterTarget>, Without<Player>)>,
    character_query_target: Query<(Entity, &Character), With<CharacterTarget>>,
) {
    for _ in ev_target_next_enemy.read() {
        let mut new_target: Option<Entity> = None;
        for (handle, _character) in character_query.iter() {
            new_target = Some(handle);
        }

        for (e, _c) in character_query_target.iter() {
            match commands.get_entity(e) {
                Some(mut ec) => {
                    ec.remove::<CharacterTarget>();
                }
                None => {}
            }
        }

        match new_target {
            Some(target) => match commands.get_entity(target) {
                Some(mut entity) => {
                    entity.insert(CharacterTarget);
                }
                None => {}
            },
            None => {}
        }
    }
}

fn keyboard_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_target_next_enemy: EventWriter<TargetNextEnemyEvent>,
    player_query: Query<Entity, With<Player>>,
    target_query: Query<Entity, With<CharacterTarget>>,
) {
    // Skills
    if keys.just_pressed(KeyCode::ShiftLeft) {
        let e = get_single!(player_query);
        commands.entity(e).insert(SprintEffect {
            timer: Timer::from_seconds(5., TimerMode::Once),
        });
    }
    // Targeting
    if keys.just_pressed(KeyCode::Escape) {
        let e = get_single!(target_query);
        commands.entity(e).remove::<CharacterTarget>();
    }
    if keys.just_pressed(KeyCode::Tab) {
        ev_target_next_enemy.send(TargetNextEnemyEvent);
    }
}

fn attack_input(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    player_query: Query<Entity, (With<Player>, Without<AttackCooldown>)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let player_handle = get_single!(player_query);
        commands.entity(player_handle).insert(AttackWindUp {
            ev: AttackEvent::new(player_handle, 2),
            total_duration: Duration::from_millis(900),
            timer: Timer::from_seconds(0.9, TimerMode::Once),
        });
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TargetNextEnemyEvent>().add_systems(
            Update,
            (keyboard_input, attack_input, handle_target_next_enemy)
                .run_if(in_state(AppState::Game)),
        );
    }
}
