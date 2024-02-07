use bevy::prelude::*;

use crate::{
    app_state::AppState,
    main_menu::UiFont,
    ui_style::nameplate_text_style,
    world3d::{Character, CharacterUI, HasCharacterUI, Player, PlayerCamera},
};

pub fn setup_character_ui(
    mut commands: Commands,
    ui_font: Res<UiFont>,
    character_query: Query<(Entity, &Character), (With<Character>, Without<HasCharacterUI>)>,
) {
    for (character_handle, character) in character_query.iter() {
        match commands.get_entity(character_handle) {
            Some(mut character_entity) => {
                character_entity.insert(HasCharacterUI);
            }
            None => {
                println!("Failed to get entity {:?}", character_handle);
            }
        }
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Px(40.),
                        width: Val::Px(160.),
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                CharacterUI(character_handle),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_sections([TextSection::new(
                    character.0.name.clone(),
                    nameplate_text_style(ui_font.0.clone()),
                )]));
                parent.spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(12.),
                        width: Val::Percent(100.),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GREEN),
                    ..default()
                });
            });
    }
}

pub fn update_character_ui(
    mut text_query: Query<(&mut Style, &CharacterUI)>,
    character_query: Query<&Transform, With<Character>>,
    player_camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
) {
    for (camera, camera_transform) in player_camera_query.iter() {
        for (mut style, character) in text_query.iter_mut() {
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
                Err(_) => {}
            }
        }
    }
}

pub fn toggle_character_ui_based_on_distance(
    mut character_ui_query: Query<(&mut Style, &CharacterUI)>,
    character_query: Query<&Transform, With<Character>>,
    player_query: Query<&Transform, With<Player>>,
) {
    match player_query.get_single() {
        Ok(player_transform) => {
            for (mut ui_style, character_ui) in character_ui_query.iter_mut() {
                match character_query.get(character_ui.0) {
                    Ok(character_transform) => {
                        let distance = player_transform
                            .translation
                            .distance(character_transform.translation);

                        match distance > 15.0 {
                            true => ui_style.display = Display::None,
                            false => ui_style.display = Display::Flex,
                        }
                    }
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
                setup_character_ui,
                update_character_ui,
                toggle_character_ui_based_on_distance,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
