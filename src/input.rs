use std::time::Duration;

use bevy::prelude::*;

use crate::{
    app_state::AppState,
    combat::{
        attack::{AttackCooldown, AttackEvent, AttackWindUp},
        status_effect::sprint::SprintEffect,
    },
    get_single,
    movement::{StrafeDirection, Strafing, TurnDirection, Turning, WalkDirection, Walking},
    world3d::{Character, Player, PlayerTarget},
};

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

fn keyboard_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_target_next_enemy: EventWriter<TargetNextEnemyEvent>,
    player_query: Query<Entity, With<Player>>,
    target_query: Query<Entity, With<PlayerTarget>>,
) {
    // Skills
    if keys.just_pressed(KeyCode::Space) {
        let e = get_single!(player_query);
        commands.entity(e).insert(SprintEffect {
            timer: Timer::from_seconds(5., TimerMode::Once),
        });
    }
    // Targeting
    if keys.just_pressed(KeyCode::Escape) {
        let e = get_single!(target_query);
        commands.entity(e).remove::<PlayerTarget>();
    }
    if keys.just_pressed(KeyCode::Tab) {
        ev_target_next_enemy.send(TargetNextEnemyEvent);
    }
}

fn attack_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, (With<Player>, Without<AttackCooldown>)>,
    player_target_query: Query<Entity, With<PlayerTarget>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        let player_target_handle = get_single!(player_target_query);
        let player_handle = get_single!(player_query);
        commands.entity(player_handle).insert(AttackWindUp {
            ev: AttackEvent::new(player_handle, player_target_handle, 5),
            total_duration: Duration::from_millis(1000),
            timer: Timer::from_seconds(1., TimerMode::Once),
        });
    }
}

fn walking_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    not_walking_query: Query<Entity, (With<Player>, (Without<Walking>, Without<AttackWindUp>))>,
    walking_query: Query<Entity, (With<Player>, With<Walking>)>,
) {
    if keys.just_pressed(KeyCode::KeyW) {
        let e = get_single!(not_walking_query);
        commands.entity(e).insert(Walking(WalkDirection::Forward));
    }
    if keys.just_pressed(KeyCode::KeyS) {
        let e = get_single!(not_walking_query);
        commands.entity(e).insert(Walking(WalkDirection::Backward));
    }
    if keys.any_just_released([KeyCode::KeyW, KeyCode::KeyS]) {
        let e = get_single!(walking_query);
        commands.entity(e).remove::<Walking>();
    }
}

fn strafing_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    not_strafing_query: Query<Entity, (With<Player>, (Without<Strafing>, Without<AttackWindUp>))>,
    strafing_query: Query<Entity, (With<Player>, With<Strafing>)>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        let e = get_single!(not_strafing_query);
        commands.entity(e).insert(Strafing(StrafeDirection::Left));
    }
    if keys.just_pressed(KeyCode::KeyE) {
        let e = get_single!(not_strafing_query);
        commands.entity(e).insert(Strafing(StrafeDirection::Right));
    }
    if keys.any_just_released([KeyCode::KeyQ, KeyCode::KeyE]) {
        let e = get_single!(strafing_query);
        commands.entity(e).remove::<Strafing>();
    }
}

fn turning_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    not_turning_query: Query<Entity, (With<Player>, (Without<Turning>, Without<AttackWindUp>))>,
    turning_query: Query<Entity, (With<Player>, With<Turning>)>,
) {
    if keys.just_pressed(KeyCode::KeyA) {
        let e = get_single!(not_turning_query);
        commands.entity(e).insert(Turning(TurnDirection::Left));
    }
    if keys.just_pressed(KeyCode::KeyD) {
        let e = get_single!(not_turning_query);
        commands.entity(e).insert(Turning(TurnDirection::Right));
    }
    if keys.any_just_released([KeyCode::KeyA, KeyCode::KeyD]) {
        let e = get_single!(turning_query);
        commands.entity(e).remove::<Turning>();
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TargetNextEnemyEvent>().add_systems(
            Update,
            (
                // walking_input,
                // strafing_input,
                // turning_input,
                keyboard_input,
                attack_input,
                handle_target_next_enemy,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
