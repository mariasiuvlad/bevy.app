use std::time::Duration;

use bevy::prelude::*;

use super::DamageTakenEvent;

#[derive(Component)]
pub struct AttackCooldown {
    pub total_duration: Duration,
    pub timer: Timer,
}

#[derive(Event, Debug)]
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

pub fn handle_attack_cooldown(
    mut commands: Commands,
    mut q: Query<(Entity, &mut AttackCooldown)>,
    time: Res<Time>,
) {
    for (entity, mut cooldown) in q.iter_mut() {
        // timers gotta be ticked, to work
        cooldown.timer.tick(time.delta());

        if cooldown.timer.finished() {
            // info!("Removing {:?}", cooldown);
            commands.entity(entity).remove::<AttackCooldown>();
        }
    }
}

pub fn handle_attack(
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_damage_taken: EventWriter<DamageTakenEvent>,
) {
    for attack_event in ev_attack.read() {
        ev_damage_taken.send(DamageTakenEvent(attack_event.attack, attack_event.target));
    }
}
