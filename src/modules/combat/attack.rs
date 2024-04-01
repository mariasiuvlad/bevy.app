use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

use crate::app_state::AppState;

use super::{
    hitbox_bundle::{handle_collision_events, handle_lifespan, HitboxBundle},
    DamageTakenEvent,
};

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

#[derive(Component, Debug)]
pub struct Hitbox(AttackEvent);

#[derive(Event, Debug, Clone, Copy)]
pub struct AttackEvent {
    pub source: Entity,
    pub attack: i32,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct HitEvent {
    pub source: Entity,
    pub target: Entity,
    pub attack: i32,
}

impl AttackEvent {
    pub fn new(source: Entity, attack: i32) -> Self {
        Self { source, attack }
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
    transform_q: Query<&Transform>,
) {
    for attack_event in ev_attack.read() {
        commands.entity(attack_event.source).insert(AttackCooldown {
            total_duration: Duration::from_secs(1),
            timer: Timer::from_seconds(1., TimerMode::Once),
        });

        if let Ok(transform) = transform_q.get(attack_event.source) {
            let mut hitbox_transform = transform.clone();
            hitbox_transform.translation += hitbox_transform.forward() * 2. + Vec3::Y;

            commands.spawn(HitboxBundle::new(
                TransformBundle::from_transform(hitbox_transform),
                Collider::cuboid(0.2, 0.2, 0.8),
                *attack_event,
                Timer::from_seconds(0.1, TimerMode::Once),
            ));
        }
    }
}

pub fn handle_hit(mut ev_hit: EventReader<HitEvent>, mut ev_damage: EventWriter<DamageTakenEvent>) {
    for ev in ev_hit.read() {
        info!("hit event: {ev:?}");
        ev_damage.send(DamageTakenEvent(ev.attack, ev.target));
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_event::<HitEvent>()
            .add_systems(
                Update,
                (
                    handle_collision_events,
                    handle_attack_windup,
                    handle_attack,
                    handle_hit,
                    handle_lifespan,
                    handle_attack_cooldown,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
