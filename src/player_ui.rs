use crate::{
    app_state::AppState,
    combat::{Energy, Health, MainPlayer, MaxEnergy, MaxHealth},
    main_menu::UiFont,
    ui_style::default_text_style,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerUiRoot;

#[derive(Component)]
pub struct PlayerUiHealthValue;

#[derive(Component)]
pub struct PlayerUiHealthBar;

#[derive(Component)]
pub struct PlayerUiEnergyValue;

#[derive(Component)]
pub struct PlayerUiEnergyBar;

pub fn setup_player_ui(mut commands: Commands, ui_font: Res<UiFont>) {
    commands
        .spawn((
            PlayerUiRoot,
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(5.)),
                    width: Val::Px(300.),
                    height: Val::Px(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PlayerUiHealthBar,
                    NodeBundle {
                        style: Style {
                            height: Val::Px(40.),
                            width: Val::Percent(100.),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::DARK_GREEN),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        PlayerUiHealthValue,
                        TextBundle::from_sections([TextSection::new(
                            "",
                            default_text_style(ui_font.0.clone()),
                        )]),
                    ));
                });
            parent
                .spawn((
                    PlayerUiEnergyBar,
                    NodeBundle {
                        style: Style {
                            height: Val::Px(40.),
                            width: Val::Percent(100.),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLUE),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        PlayerUiEnergyValue,
                        TextBundle::from_sections([TextSection::new(
                            "",
                            default_text_style(ui_font.0.clone()),
                        )]),
                    ));
                });
        });
}

pub fn update_player_ui_health(
    player_query: Query<&Health, With<MainPlayer>>,
    mut health_value_ui_query: Query<&mut Text, With<PlayerUiHealthValue>>,
) {
    let health = player_query.get_single().unwrap();
    let mut health_text = health_value_ui_query.get_single_mut().unwrap();
    health_text.sections[0].value = health.0.to_string().into();
}

pub fn update_player_ui_energy(
    player_query: Query<&Energy, With<MainPlayer>>,
    mut energy_value_ui_query: Query<&mut Text, With<PlayerUiEnergyValue>>,
) {
    let energy = player_query.get_single().unwrap();
    let mut energy_text = energy_value_ui_query.get_single_mut().unwrap();
    energy_text.sections[0].value = energy.0.to_string().into();
}

pub fn update_player_ui_health_bar(
    player_query: Query<(&Health, &MaxHealth), With<MainPlayer>>,
    mut health_bar_ui_query: Query<&mut Style, With<PlayerUiHealthBar>>,
) {
    let (health, max_health) = player_query.get_single().unwrap();
    let percent = health.0 as f32 * 100.0 / max_health.0 as f32;
    let mut health_bar = health_bar_ui_query.get_single_mut().unwrap();
    health_bar.width = Val::Percent(percent);
}

pub fn update_player_ui_energy_bar(
    player_query: Query<(&Energy, &MaxEnergy), With<MainPlayer>>,
    mut energy_bar_ui_query: Query<&mut Style, With<PlayerUiEnergyBar>>,
) {
    let (energy, max_energy) = player_query.get_single().unwrap();
    let percent = energy.0 as f32 * 100.0 / max_energy.0 as f32;
    let mut energy_bar = energy_bar_ui_query.get_single_mut().unwrap();
    energy_bar.width = Val::Percent(percent);
}

pub struct PlayerUiPlugin;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_player_ui)
            .add_systems(
                Update,
                (
                    update_player_ui_health,
                    update_player_ui_energy,
                    update_player_ui_health_bar,
                    update_player_ui_energy_bar,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
