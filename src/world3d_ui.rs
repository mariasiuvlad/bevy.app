use bevy::prelude::*;

use crate::{
    app_state::AppState,
    combat::{Health, MaxHealth},
    main_menu::UiFont,
    ui_style::nameplate_text_style,
    world3d::{Character, Player, PlayerCamera, PlayerTarget},
};

#[derive(Component)]
pub struct PlayerTargetUI(pub Entity);

#[derive(Component)]
pub struct CharacterUI(pub Entity);
#[derive(Component)]
pub struct HealthBarUI(pub Entity);

pub fn setup_nameplates(
    mut commands: Commands,
    ui_font: Res<UiFont>,
    character_query: Query<(Entity, &Character), Added<Character>>,
) {
    for (character_handle, character) in character_query.iter() {
        let nameplate_handle = commands
            .spawn((
                CharacterUI(character_handle),
                NodeBundle {
                    style: Style {
                        height: Val::Auto,
                        width: Val::Px(160.),
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
                        nameplate_text_style(ui_font.0.clone()),
                    )]),
                ));
                parent.spawn(TextBundle::from_sections([TextSection::new(
                    character.0.name.clone(),
                    nameplate_text_style(ui_font.0.clone()),
                )]));
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(12.),
                            width: Val::Percent(100.),
                            border: UiRect::all(Val::Px(2.)),
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
    character_query: Query<(&Health, &MaxHealth), With<Character>>,
) {
    for (mut style, health_bar_ui) in health_bar_ui_query.iter_mut() {
        if let Ok((health, max_health)) = character_query.get(health_bar_ui.0) {
            let current_health_percentage = (health.0 * 100) as f32 / max_health.0 as f32;
            style.width = Val::Percent(current_health_percentage);
        }
    }
}

pub fn update_nameplates_position(
    mut commands: Commands,
    mut character_ui_query: Query<(Entity, &mut Style, &CharacterUI)>,
    character_query: Query<&Transform, With<Character>>,
    player_camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
) {
    if let Ok((camera, camera_transform)) = player_camera_query.get_single() {
        for (ui_handle, mut style, character) in character_ui_query.iter_mut() {
            match character_query.get(character.0) {
                Ok(character_transform) => {
                    match camera
                        .world_to_viewport(camera_transform, character_transform.translation)
                    {
                        Some(coords) => {
                            style.left = Val::Px(coords.x - 80.);
                            style.top = Val::Px(coords.y - 80.);
                        }
                        None => {}
                    }
                }
                Err(_) => {
                    info!("Should despawn ui!");
                    commands.entity(ui_handle).despawn_recursive();
                }
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
    character_query: Query<(&Transform, Option<&PlayerTarget>), With<Character>>,
    player_query: Query<&Transform, With<Player>>,
) {
    match player_query.get_single() {
        Ok(player_transform) => {
            for (mut ui_style, character_ui) in character_ui_query.iter_mut() {
                match character_query.get(character_ui.0) {
                    Ok((character_transform, player_target)) => match player_target {
                        Some(_) => {
                            ui_style.display = Display::Flex;
                        }
                        None => {
                            let distance = player_transform
                                .translation
                                .distance(character_transform.translation);

                            match distance > 15.0 {
                                true => ui_style.display = Display::None,
                                false => ui_style.display = Display::Flex,
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
        }
        Err(_) => {
            println!("Failed to find player");
        }
    }
}

pub struct World3dUiPlugin;

impl Plugin for World3dUiPlugin {
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
