use crate::{
    app_state::AppState,
    combat::combat_stats::Stats,
    main_menu::UiFont,
    ui_style::player_ui_text_style,
    world3d::{Player, PlayerTarget},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct UiRoot<T: Component>(T);

#[derive(Component)]
pub struct UiHealthValue<T: Component>(T);

#[derive(Component)]
pub struct UiHealthPercentage<T: Component>(T);

#[derive(Component)]
pub struct UiEnergyValue<T: Component>(T);

#[derive(Component)]
pub struct UiEnergyPercentage<T: Component>(T);

pub fn setup_ui(mut commands: Commands, ui_font: Res<UiFont>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    UiRoot(Player),
                    NodeBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(16.)),
                            width: Val::Px(300.),
                            height: Val::Px(40.),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            UiHealthPercentage(Player),
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    flex_grow: 1.,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::DARK_GREEN),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                UiHealthValue(Player),
                                TextBundle::from_sections([TextSection::new(
                                    "",
                                    player_ui_text_style(ui_font.0.clone()),
                                )]),
                            ));
                        });
                    parent
                        .spawn((
                            UiEnergyPercentage(Player),
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    flex_grow: 1.,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::BLUE),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                UiEnergyValue(Player),
                                TextBundle::from_sections([TextSection::new(
                                    "",
                                    player_ui_text_style(ui_font.0.clone()),
                                )]),
                            ));
                        });
                });
            parent
                .spawn((
                    UiRoot(PlayerTarget),
                    NodeBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(16.)),
                            width: Val::Px(300.),
                            height: Val::Px(40.),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            UiHealthPercentage(PlayerTarget),
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    flex_grow: 1.,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::DARK_GREEN),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                UiHealthValue(PlayerTarget),
                                TextBundle::from_sections([TextSection::new(
                                    "",
                                    player_ui_text_style(ui_font.0.clone()),
                                )]),
                            ));
                        });
                    parent
                        .spawn((
                            UiEnergyPercentage(PlayerTarget),
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    flex_grow: 1.,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::BLUE),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                UiEnergyValue(PlayerTarget),
                                TextBundle::from_sections([TextSection::new(
                                    "",
                                    player_ui_text_style(ui_font.0.clone()),
                                )]),
                            ));
                        });
                });
        });
}

pub fn update_health_value<T: Component>(
    character_query: Query<&Stats, With<T>>,
    mut value_ui_query: Query<&mut Text, With<UiHealthValue<T>>>,
) {
    if let Ok(stats) = character_query.get_single() {
        if let Ok(mut health_text) = value_ui_query.get_single_mut() {
            health_text.sections[0].value = stats.health.to_string().into();
        }
    }
}

pub fn update_energy_value<T: Component>(
    player_query: Query<&Stats, With<T>>,
    mut energy_value_ui_query: Query<&mut Text, With<UiEnergyValue<T>>>,
) {
    if let Ok(stats) = player_query.get_single() {
        if let Ok(mut energy_text) = energy_value_ui_query.get_single_mut() {
            energy_text.sections[0].value = stats.energy.to_string().into();
        }
    }
}

pub fn update_health_percentage<T: Component>(
    character_query: Query<&Stats, With<T>>,
    mut bar_ui_query: Query<&mut Style, With<UiHealthPercentage<T>>>,
) {
    if let Ok(stats) = character_query.get_single() {
        if let Ok(mut bar) = bar_ui_query.get_single_mut() {
            bar.width = Val::Percent(stats.health_percentage());
        }
    }
}

pub fn update_energy_percentage<T: Component>(
    character_query: Query<&Stats, With<T>>,
    mut bar_ui_query: Query<&mut Style, With<UiEnergyPercentage<T>>>,
) {
    if let Ok(stats) = character_query.get_single() {
        if let Ok(mut bar) = bar_ui_query.get_single_mut() {
            bar.width = Val::Percent(stats.energy_percentage());
        }
    }
}

pub fn toggle_ui<T: Component>(
    target_query: Query<Entity, With<T>>,
    mut ui_query: Query<&mut Style, With<UiRoot<T>>>,
) {
    if let Ok(mut style) = ui_query.get_single_mut() {
        match target_query.get_single() {
            Err(_) => style.display = Display::None,
            Ok(_) => style.display = Display::Flex,
        }
    }
}

pub struct PlayerUiPlugin;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_ui)
            .add_systems(
                Update,
                (
                    update_health_value::<Player>,
                    update_energy_value::<Player>,
                    update_health_percentage::<Player>,
                    update_energy_percentage::<Player>,
                    update_health_value::<PlayerTarget>,
                    update_energy_value::<PlayerTarget>,
                    update_health_percentage::<PlayerTarget>,
                    update_energy_percentage::<PlayerTarget>,
                    toggle_ui::<PlayerTarget>,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
