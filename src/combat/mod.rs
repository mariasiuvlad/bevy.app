use bevy::prelude::*;

use crate::{app_state::AppState, world3d::Character};

use self::{
    attack::{handle_attack, handle_attack_cooldown, AttackEvent},
    combat_stats::Stats,
    status_effect::{sprint::SprintPlugin, thorns::ThornsPlugin},
};

pub mod attack;
pub mod combat_stats;
pub mod status_effect;

#[derive(Event, Debug)]
pub struct DamageTakenEvent(pub i32, pub Entity);

#[derive(Event, Debug)]
pub struct CharacterDeathEvent(pub Entity);

fn handle_damage_taken(
    mut ev_damage: EventReader<DamageTakenEvent>,
    mut character_query: Query<&mut Stats, With<Character>>,
) {
    for ev in ev_damage.read() {
        if let Ok(mut stats) = character_query.get_mut(ev.1) {
            stats.health -= ev.0;
        }
    }
}

fn handle_health_change(
    mut ev_death: EventWriter<CharacterDeathEvent>,
    character_query: Query<(Entity, &Stats), Changed<Stats>>,
) {
    for (e, stats) in character_query.iter() {
        if stats.health <= 0 {
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

fn log_combat(
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_damage: EventReader<DamageTakenEvent>,
    character_query: Query<&Character>,
) {
    for attack_event in ev_attack.read() {
        let source = character_query.get(attack_event.source).unwrap();
        let target = character_query.get(attack_event.target).unwrap();

        info!(
            "{} attacks {} for {}",
            source.0.name, target.0.name, attack_event.attack
        );
    }

    for damage_event in ev_damage.read() {
        let source = character_query.get(damage_event.1).unwrap();
        info!("{} takes {} damage", source.0.name, damage_event.0);
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageTakenEvent>()
            .add_event::<AttackEvent>()
            .add_event::<CharacterDeathEvent>()
            .add_plugins((SprintPlugin, ThornsPlugin))
            .add_systems(
                Update,
                (
                    handle_damage_taken,
                    handle_attack,
                    handle_attack_cooldown,
                    log_combat,
                    handle_health_change,
                    handle_death,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}