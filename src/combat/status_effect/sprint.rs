use bevy::prelude::*;

use crate::{app_state::AppState, combat::combat_stats::Stats, world3d::Character};

#[derive(Component, Debug)]
pub struct SprintEffect {
    pub timer: Timer,
}

pub fn elapse_status_effects(
    mut commands: Commands,
    mut q: Query<(Entity, &mut SprintEffect)>,
    time: Res<Time>,
) {
    for (entity, mut status_effect) in q.iter_mut() {
        status_effect.timer.tick(time.delta());

        if status_effect.timer.finished() {
            info!("Removing {:?}", status_effect);
            commands.entity(entity).remove::<SprintEffect>();
        }
    }
}

pub fn handle_status_effects(
    mut status_effects_query: Query<(&SprintEffect, &mut Stats), Added<SprintEffect>>,
) {
    for (_, mut stats) in status_effects_query.iter_mut() {
        stats.move_speed_modifier += 1.
    }
}

pub fn handle_status_effects_removals(
    mut removals: RemovedComponents<SprintEffect>,
    mut character_query: Query<&mut Stats, With<Character>>,
) {
    for entity in removals.read() {
        if let Ok(mut stats) = character_query.get_mut(entity) {
            stats.move_speed_modifier -= 1.;
        }
    }
}

pub struct SprintPlugin;

impl Plugin for SprintPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (elapse_status_effects, handle_status_effects)
                    .before(handle_status_effects_removals),
                handle_status_effects_removals,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
