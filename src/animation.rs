use std::time::Duration;

use bevy::prelude::*;

use crate::{
    app_state::AppState,
    combat::{attack::AttackWindUp, status_effect::sprint::SprintEffect, DamageTakenEvent},
    movement::{WalkDirection, Walking},
    startup::Animations,
    world3d::Player,
};

// Once the scene is loaded, start the animation
pub fn start_idle(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.idle.clone_weak()).repeat();
    }
}

pub fn start_walking_animation(
    animations: Res<Animations>,
    walking_query: Query<(Entity, &Walking, Option<&SprintEffect>), Added<Walking>>,
    mut players: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
) {
    for (walking_entity, walking, sprint) in walking_query.iter() {
        for e in children.iter_descendants(walking_entity) {
            if let Ok(mut player) = players.get_mut(e) {
                let animation_handle = match walking.0 {
                    WalkDirection::Backward => animations.backpedal.clone_weak(),
                    WalkDirection::Forward => match sprint {
                        None => animations.walk.clone_weak(),
                        Some(_) => animations.run.clone_weak(),
                    },
                };
                player
                    .start_with_transition(animation_handle, Duration::from_millis(500))
                    .repeat();
            }
        }
    }
}

fn stop_walking_animation(
    animations: Res<Animations>,
    mut stopped_walking: RemovedComponents<Walking>,
    mut stopped_attacking: RemovedComponents<AttackWindUp>,
    mut animation_players: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
) {
    for e in stopped_walking.read() {
        // do something with the entity
        for child in children.iter_descendants(e) {
            if let Ok(mut player) = animation_players.get_mut(child) {
                player
                    .start_with_transition(animations.idle.clone_weak(), Duration::from_millis(250))
                    .repeat();
            }
        }
    }

    for e in stopped_attacking.read() {
        // do something with the entity
        for child in children.iter_descendants(e) {
            if let Ok(mut player) = animation_players.get_mut(child) {
                player
                    .start_with_transition(animations.idle.clone_weak(), Duration::from_millis(500))
                    .repeat();
            }
        }
    }
}

fn start_attack_animation(
    attacking_player_query: Query<Entity, (With<Player>, Added<AttackWindUp>)>,
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
) {
    if let Ok(player_handle) = attacking_player_query.get_single() {
        for e in children.iter_descendants(player_handle) {
            if let Ok(mut player) = players.get_mut(e) {
                player.start(animations.attack.clone_weak());
            }
        }
    }
}

fn start_flinch_animation(
    mut ev_damage: EventReader<DamageTakenEvent>,
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
) {
    for ev in ev_damage.read() {
        for e in children.iter_descendants(ev.1) {
            if let Ok(mut player) = players.get_mut(e) {
                player.start(animations.flinch.clone_weak());
            }
        }
    }
}

fn start_sprint_animation(
    sprinting_player_query: Query<Entity, (With<Walking>, Added<SprintEffect>)>,
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
) {
    for sprinting_e in sprinting_player_query.iter() {
        for e in children.iter_descendants(sprinting_e) {
            if let Ok(mut player) = players.get_mut(e) {
                player
                    .start_with_transition(animations.run.clone_weak(), Duration::from_millis(500))
                    .repeat();
            }
        }
    }
}

fn stop_sprinting_animation(
    animations: Res<Animations>,
    mut stopped_sprinting: RemovedComponents<SprintEffect>,
    is_walking: Query<Entity, With<Walking>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
) {
    for e in stopped_sprinting.read() {
        if let Ok(en) = is_walking.get(e) {
            for child in children.iter_descendants(en) {
                if let Ok(mut player) = animation_players.get_mut(child) {
                    player
                        .start_with_transition(
                            animations.walk.clone_weak(),
                            Duration::from_millis(500),
                        )
                        .repeat();
                }
            }
        }
    }
}

pub struct CharacterAnimationPlugin;

impl Plugin for CharacterAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_idle,
                start_walking_animation,
                stop_walking_animation,
                start_attack_animation,
                start_flinch_animation,
                start_sprint_animation,
                stop_sprinting_animation,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
