use bevy::prelude::*;

use crate::{app_state::AppState, world3d::Character};

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct MaxHealth(pub i32);

#[derive(Component)]
pub struct Energy(pub i32);

#[derive(Component)]
pub struct MaxEnergy(pub i32);

#[derive(Bundle)]
pub struct CombatStatsBundle {
    max_health: MaxHealth,
    health: Health,
    max_energy: MaxEnergy,
    energy: Energy,
}

impl Default for CombatStatsBundle {
    fn default() -> Self {
        CombatStatsBundle {
            max_health: MaxHealth(20),
            health: Health(20),
            max_energy: MaxEnergy(100),
            energy: Energy(100),
        }
    }
}

#[derive(Event, Debug)]
pub struct DamageTakenEvent(pub i32, pub Entity);

#[derive(Event, Debug)]
pub struct CharacterDeathEvent(pub Entity);

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
    mut ev_death: EventWriter<CharacterDeathEvent>,
    character_query: Query<(Entity, &Health), Changed<Health>>,
) {
    for (e, h) in character_query.iter() {
        if h.0 <= 0 {
            ev_death.send(CharacterDeathEvent(e));
        }
    }
}

fn handle_death(
    mut commands: Commands,
    mut ev_death: EventReader<CharacterDeathEvent>,
    character_query: Query<Entity, With<Character>>,
) {
    for ev in ev_death.read() {
        if let Ok(character_handle) = character_query.get(ev.0) {
            commands.entity(character_handle).despawn();
        }
    }
}

fn handle_attack(
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_damage_taken: EventWriter<DamageTakenEvent>,
) {
    for attack_event in ev_attack.read() {
        ev_damage_taken.send(DamageTakenEvent(attack_event.attack, attack_event.target));
    }
}

fn log_attack(mut ev_attack: EventReader<AttackEvent>, character_query: Query<&Character>) {
    for attack_event in ev_attack.read() {
        let source = character_query.get(attack_event.source).unwrap();
        let target = character_query.get(attack_event.target).unwrap();

        info!(
            "{} attacks {} for {}",
            source.0.name, target.0.name, attack_event.attack
        );
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageTakenEvent>()
            .add_event::<AttackEvent>()
            .add_event::<CharacterDeathEvent>()
            .add_systems(
                Update,
                (
                    handle_damage_taken,
                    handle_attack,
                    log_attack,
                    handle_health_change,
                    handle_death,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
