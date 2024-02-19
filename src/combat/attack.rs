use std::time::Duration;

use bevy::prelude::*;

use crate::app_state::AppState;

use super::DamageTakenEvent;

#[derive(Component)]
pub struct AttackWindUp {
    pub ev: AttackEvent,
    pub total_duration: Duration,
    pub timer: Timer,
}

#[derive(Component)]
pub struct AttackCooldown {
    pub total_duration: Duration,
    pub timer: Timer,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct AttackEvent {
    pub source: Entity,
    pub target: Entity,
    pub attack: i32,
}

impl AttackEvent {
    pub fn new(source: Entity, target: Entity, attack: i32) -> Self {
        Self {
            source,
            target,
            attack,
        }
    }
}

pub fn handle_attack_windup(
    mut commands: Commands,
    mut q: Query<&mut AttackWindUp>,
    mut ev_attack: EventWriter<AttackEvent>,
    time: Res<Time>,
) {
    for mut attack_wind_up in q.iter_mut() {
        attack_wind_up.timer.tick(time.delta());
        if attack_wind_up.timer.finished() {
            commands
                .entity(attack_wind_up.ev.source)
                .remove::<AttackWindUp>();
            ev_attack.send(attack_wind_up.ev);
        }
    }
}

pub fn handle_attack_cooldown(
    mut commands: Commands,
    mut q: Query<(Entity, &mut AttackCooldown)>,
    time: Res<Time>,
) {
    for (entity, mut cooldown) in q.iter_mut() {
        cooldown.timer.tick(time.delta());

        if cooldown.timer.finished() {
            commands.entity(entity).remove::<AttackCooldown>();
        }
    }
}

pub fn handle_attack(
    mut commands: Commands,
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_damage_taken: EventWriter<DamageTakenEvent>,
) {
    for attack_event in ev_attack.read() {
        ev_damage_taken.send(DamageTakenEvent(attack_event.attack, attack_event.target));

        commands.entity(attack_event.source).insert(AttackCooldown {
            total_duration: Duration::from_secs(1),
            timer: Timer::from_seconds(1., TimerMode::Once),
        });
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>().add_systems(
            Update,
            (handle_attack_windup, handle_attack, handle_attack_cooldown)
                .run_if(in_state(AppState::Game)),
        );
    }
}
