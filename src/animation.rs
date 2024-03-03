use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animated_bundle::{AnimationState, AnimationStates, ModelAnimations},
    app_state::AppState,
    combat::{attack::AttackWindUp, status_effect::sprint::SprintEffect, DamageTakenEvent},
    movement::{WalkDirection, Walking},
};

pub fn start_animation(
    animatables: Query<(Entity, &AnimationState, &ModelAnimations)>,
    mut added_players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
    children: Query<&Children>,
) {
    for (e, animation_state, animations) in animatables.iter() {
        for e in children.iter_descendants(e) {
            if let Ok(mut player) = added_players.get_mut(e) {
                player
                    .start(animations.match_animation_state(animation_state))
                    .repeat();
            }
        }
    }
}

pub fn handle_animation_change(
    models: Query<(Entity, &ModelAnimations, &AnimationState), Changed<AnimationState>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
) {
    for (model_entity, animations, animation_state) in models.iter() {
        for e in children.iter_descendants(model_entity) {
            if let Ok(mut animation_player) = animation_players.get_mut(e) {
                animation_player
                    .start_with_transition(
                        animations.match_animation_state(animation_state),
                        Duration::from_millis(250),
                    )
                    .repeat();
            }
        }
    }
}

pub fn start_walking_animation(
    mut commands: Commands,
    walking_query: Query<(Entity, &Walking, Option<&SprintEffect>), Added<Walking>>,
) {
    for (walking_entity, walking, sprint) in walking_query.iter() {
        let animation_state = match walking.0 {
            WalkDirection::Backward => AnimationStates::Backpedal,
            WalkDirection::Forward => match sprint {
                None => AnimationStates::Walk,
                Some(_) => AnimationStates::Run,
            },
        };

        commands
            .entity(walking_entity)
            .insert(AnimationState(animation_state));
    }
}

fn stop_walking_animation(
    mut commands: Commands,
    mut stopped_walking: RemovedComponents<Walking>,
    mut stopped_attacking: RemovedComponents<AttackWindUp>,
) {
    for e in stopped_walking.read() {
        commands
            .entity(e)
            .insert(AnimationState(AnimationStates::Idle));
    }

    for e in stopped_attacking.read() {
        commands
            .entity(e)
            .insert(AnimationState(AnimationStates::Idle));
    }
}

fn start_attack_animation(
    mut commands: Commands,
    attacking_player_query: Query<Entity, Added<AttackWindUp>>,
) {
    for player_handle in attacking_player_query.iter() {
        commands
            .entity(player_handle)
            .insert(AnimationState(AnimationStates::Attack));
    }
}

fn start_flinch_animation(mut commands: Commands, mut ev_damage: EventReader<DamageTakenEvent>) {
    for ev in ev_damage.read() {
        commands
            .entity(ev.1)
            .insert(AnimationState(AnimationStates::Flinch));
    }
}

fn start_sprint_animation(
    mut commands: Commands,
    sprinting_player_query: Query<Entity, (With<Walking>, Added<SprintEffect>)>,
) {
    for e in sprinting_player_query.iter() {
        commands
            .entity(e)
            .insert(AnimationState(AnimationStates::Run));
    }
}

fn stop_sprint_animation(
    mut commands: Commands,
    mut stopped_sprinting: RemovedComponents<SprintEffect>,
    is_walking: Query<Entity, With<Walking>>,
) {
    for e in stopped_sprinting.read() {
        if let Ok(e) = is_walking.get(e) {
            commands
                .entity(e)
                .insert(AnimationState(AnimationStates::Walk));
        }
    }
}

pub struct CharacterAnimationPlugin;

impl Plugin for CharacterAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_animation,
                handle_animation_change,
                start_walking_animation,
                stop_walking_animation,
                start_attack_animation,
                start_flinch_animation,
                start_sprint_animation,
                stop_sprint_animation,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
