use bevy::prelude::*;

use crate::{
    app_state::AppState,
    combat::{attack::AttackEvent, DamageTakenEvent},
};

#[derive(Component, Debug)]
pub struct ThornsEffect {
    pub timer: Timer,
}

pub fn elapse_thorns_effects(
    mut commands: Commands,
    mut q: Query<(Entity, &mut ThornsEffect)>,
    time: Res<Time>,
) {
    for (entity, mut status_effect) in q.iter_mut() {
        status_effect.timer.tick(time.delta());

        if status_effect.timer.finished() {
            commands.entity(entity).remove::<ThornsEffect>();
        }
    }
}

pub fn handle_thorns_effects(
    thorns_query: Query<Entity, With<ThornsEffect>>,
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_damage: EventWriter<DamageTakenEvent>,
) {
    for ev in ev_attack.read() {
        if let Ok(_) = thorns_query.get(ev.target) {
            // target with thorns was attacked
            ev_damage.send(DamageTakenEvent(1, ev.source));
        }
    }
}

pub struct ThornsPlugin;

impl Plugin for ThornsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (elapse_thorns_effects, handle_thorns_effects).run_if(in_state(AppState::Game)),
        );
    }
}
