use bevy::prelude::*;

use crate::{
    app_state::AppState,
    get_single,
    main_menu::UiFont,
    modules::combat::combat_stats::Stats,
    world3d::{Player, PlayerCamera, PlayerTarget, Targetable},
};

use super::style;

#[derive(Component)]
pub struct CharacterUI(pub Entity);

#[derive(Component)]
pub struct PlayerTargetUI(pub Entity);

#[derive(Component)]
pub struct HealthBarUI(pub Entity);

pub fn setup_nameplates(
    mut commands: Commands,
    ui_font: Res<UiFont>,
    targetable_query: Query<(Entity, &Name), Added<Targetable>>,
) {
    for (character_handle, name) in targetable_query.iter() {
        info!("nameplate target handle {:?}", character_handle);
        let nameplate_handle = commands
            .spawn((
                CharacterUI(character_handle),
                NodeBundle {
                    style: Style {
                        height: Val::Auto,
                        width: Val::Px(160.),
                        margin: UiRect::left(Val::Px(-80.)),
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    PlayerTargetUI(character_handle),
                    TextBundle::from_sections([TextSection::new(
                        String::from(">Target<"),
                        style::nameplate_text_style(ui_font.0.clone()),
                    )]),
                ));
                parent.spawn(TextBundle::from_sections([TextSection::new(
                    name,
                    style::nameplate_text_style(ui_font.0.clone()),
                )]));
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(8.),
                            width: Val::Percent(100.),
                            border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            HealthBarUI(character_handle),
                            NodeBundle {
                                style: Style {
                                    height: Val::Percent(100.),
                                    width: Val::Percent(100.),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::DARK_GREEN),
                                ..default()
                            },
                        ));
                    });
            })
            .id();

        if let Some(mut entity_commands) = commands.get_entity(character_handle) {
            entity_commands.insert(CharacterUI(nameplate_handle));
        }
    }
}

pub fn update_nameplates_health(
    mut health_bar_ui_query: Query<(&mut Style, &HealthBarUI)>,
    character_query: Query<&Stats, With<Targetable>>,
) {
    for (mut style, health_bar_ui) in health_bar_ui_query.iter_mut() {
        if let Ok(stats) = character_query.get(health_bar_ui.0) {
            style.width = Val::Percent(stats.health_percentage());
        }
    }
}

pub fn update_nameplates_position(
    mut commands: Commands,
    mut character_ui_query: Query<(Entity, &mut Style, &CharacterUI)>,
    character_query: Query<&Transform, With<Targetable>>,
    player_camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
) {
    let (camera, camera_transform) = get_single!(player_camera_query);
    for (ui_handle, mut style, character) in character_ui_query.iter_mut() {
        match character_query.get(character.0) {
            Ok(character_transform) => {
                match camera.world_to_viewport(camera_transform, character_transform.translation) {
                    Some(coords) => {
                        style.left = Val::Px(coords.x);
                        style.top = Val::Px(coords.y);
                    }
                    None => {}
                }
            }
            Err(_) => {
                commands.entity(ui_handle).despawn_recursive();
            }
        }
    }
}

pub fn update_target_indicator(
    mut player_target_ui_query: Query<(&mut Style, &PlayerTargetUI)>,
    player_target_query: Query<Entity, With<PlayerTarget>>,
) {
    for (mut style, player_target) in player_target_ui_query.iter_mut() {
        match player_target_query.get(player_target.0) {
            Ok(_) => {
                style.display = Display::Flex;
            }
            Err(_) => {
                style.display = Display::None;
            }
        }
    }
}

pub fn toggle_nameplates_based_on_distance(
    mut character_ui_query: Query<(&mut Style, &CharacterUI)>,
    character_query: Query<(&Transform, Option<&PlayerTarget>), With<Targetable>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = get_single!(player_query);
    for (mut ui_style, character_ui) in character_ui_query.iter_mut() {
        if let Ok((character_transform, player_target)) = character_query.get(character_ui.0) {
            match player_target {
                Some(_) => {
                    ui_style.display = Display::Flex;
                }
                None => {
                    let distance = player_transform
                        .translation
                        .distance(character_transform.translation);

                    ui_style.display = match distance < 15.0 {
                        true => Display::Flex,
                        false => Display::None,
                    };
                }
            }
        }
    }
}

pub struct NameplatePlugin;

impl Plugin for NameplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_nameplates,
                update_nameplates_position,
                update_nameplates_health,
                toggle_nameplates_based_on_distance,
                update_target_indicator,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
