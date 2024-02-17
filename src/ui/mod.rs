use crate::{
    app_state::AppState,
    combat::combat_stats::Stats,
    main_menu::UiFont,
    world3d::{Player, PlayerTarget},
};
use bevy::prelude::*;

use self::common::{bar, container, text, unit_frame};

mod common;

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
    commands.spawn(container()).with_children(|parent| {
        parent
            .spawn((UiRoot(Player), unit_frame()))
            .with_children(|parent| {
                parent
                    .spawn((UiHealthPercentage(Player), bar(Color::DARK_GREEN)))
                    .with_children(|parent| {
                        parent.spawn((UiHealthValue(Player), text(ui_font.0.clone())));
                    });
                parent
                    .spawn((UiEnergyPercentage(Player), bar(Color::BLUE)))
                    .with_children(|parent| {
                        parent.spawn((UiEnergyValue(Player), text(ui_font.0.clone())));
                    });
            });
        parent
            .spawn((UiRoot(PlayerTarget), unit_frame()))
            .with_children(|parent| {
                parent
                    .spawn((UiHealthPercentage(PlayerTarget), bar(Color::DARK_GREEN)))
                    .with_children(|parent| {
                        parent.spawn((UiHealthValue(PlayerTarget), text(ui_font.0.clone())));
                    });
                parent
                    .spawn((UiEnergyPercentage(PlayerTarget), bar(Color::BLUE)))
                    .with_children(|parent| {
                        parent.spawn((UiEnergyValue(PlayerTarget), text(ui_font.0.clone())));
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
            health_text.sections[0].value = format!("{}/{}", stats.health, stats.max_health);
        }
    }
}

pub fn update_energy_value<T: Component>(
    player_query: Query<&Stats, With<T>>,
    mut energy_value_ui_query: Query<&mut Text, With<UiEnergyValue<T>>>,
) {
    if let Ok(stats) = player_query.get_single() {
        if let Ok(mut energy_text) = energy_value_ui_query.get_single_mut() {
            energy_text.sections[0].value = format!("{}/{}", stats.energy, stats.max_energy);
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

pub struct UiPlugin;

impl Plugin for UiPlugin {
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
