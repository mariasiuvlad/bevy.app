use std::fmt;

use bevy::prelude::*;

use crate::{app_state::AppState, world3d::Character};

#[derive(Component)]
pub struct PlayerUi(i32);

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct MaxHealth(pub i32);

#[derive(Component)]
pub struct Energy(pub i32);

#[derive(Component)]
pub struct MaxEnergy(pub i32);

#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct Player(pub i32);

#[derive(Event, Debug)]
pub struct DamageTakenEvent(pub i32, pub Entity);

#[derive(Event, Debug)]
pub struct AttackEvent {
    source: Entity,
    target: Entity,
    attack: i32,
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

fn handle_damage_taken(
    mut ev_damage: EventReader<DamageTakenEvent>,
    mut character_query: Query<&mut Health, With<Character>>,
) {
    for ev in ev_damage.read() {
        info!("{:?} takes {} damage", ev.1, ev.0);
        if let Ok(mut health) = character_query.get_mut(ev.1) {
            health.0 -= ev.0;
        }
    }
}

fn handle_health_change(
    mut commands: Commands,
    character_query: Query<(Entity, &Health), Changed<Health>>,
) {
    for (e, h) in character_query.iter() {
        if h.0 <= 0 {
            commands.entity(e).despawn();
        }
    }
}

fn handle_attack(
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_damage_taken: EventWriter<DamageTakenEvent>,
) {
    for attack_event in ev_attack.read() {
        println!(
            "{:?} attacks {:?} for {}",
            attack_event.source, attack_event.target, attack_event.attack
        );
        ev_damage_taken.send(DamageTakenEvent(attack_event.attack, attack_event.target));
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageTakenEvent>()
            .add_event::<AttackEvent>()
            .add_systems(
                Update,
                (handle_damage_taken, handle_attack, handle_health_change)
                    .run_if(in_state(AppState::Game)),
            );
    }
}
