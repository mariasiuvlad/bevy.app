use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animated_bundle::{AnimationState, AnimationStates, ModelAnimations},
    app_state::AppState,
    combat::{attack::AttackWindUp, DamageTakenEvent},
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

pub struct CharacterAnimationPlugin;

impl Plugin for CharacterAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_animation,
                handle_animation_change,
                start_attack_animation,
                start_flinch_animation,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
