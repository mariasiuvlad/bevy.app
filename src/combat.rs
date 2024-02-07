use std::fmt;

use bevy::prelude::*;

use crate::app_state::AppState;

#[derive(Component)]
pub struct PlayerUi(u8);

#[derive(Component)]
pub struct Health(pub u8);

#[derive(Component)]
pub struct MaxHealth(pub u8);

#[derive(Component)]
pub struct Energy(pub u8);

#[derive(Component)]
pub struct MaxEnergy(pub u8);

#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct Player(pub u8);

#[derive(Component)]
pub struct MainPlayer;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {}", self.0)
    }
}

#[derive(Event, Debug)]
pub struct DamageTakenEvent(pub u8, pub Player);

#[derive(Event, Debug)]
pub struct AttackEvent {
    source: Player,
    target: Player,
    attack: u8,
}

impl AttackEvent {
    pub fn new(source: Player, target: Player, attack: u8) -> Self {
        Self {
            source,
            target,
            attack,
        }
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        MainPlayer,
        Player(1),
        MaxHealth(20),
        Health(20),
        Energy(20),
        MaxEnergy(20),
    ));
    commands.spawn((Player(2), Health(20), Energy(10)));
}

fn update_health(
    mut ev_damage: EventReader<DamageTakenEvent>,
    mut player_query: Query<(&Player, &mut Health)>,
) {
    for ev in ev_damage.read() {
        for (player, mut health) in player_query.iter_mut() {
            if *player == ev.1 {
                health.0 -= ev.0;
            }
        }
    }
}

fn handle_attack(
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_damage_taken: EventWriter<DamageTakenEvent>,
) {
    for attack_event in ev_attack.read() {
        println!(
            "{} attacks {} for {}",
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
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (update_health, handle_attack).run_if(in_state(AppState::Game)),
            );
    }
}
