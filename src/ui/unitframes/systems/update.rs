use bevy::prelude::*;

use crate::modules::combat::combat_stats::Stats;

use super::super::components::{
    UiEnergyPercentage, UiEnergyValue, UiHealthPercentage, UiHealthValue, UiName, UiUnitFrameRoot,
};

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

pub fn update_name<T: Component>(
    name_query: Query<&Name, With<T>>,
    mut name_ui_query: Query<&mut Text, With<UiName<T>>>,
) {
    if let Ok(name) = name_query.get_single() {
        if let Ok(mut name_text) = name_ui_query.get_single_mut() {
            name_text.sections[0].value = format!("{}", name);
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

pub fn update_display<T: Component>(
    target_query: Query<Entity, With<T>>,
    mut ui_query: Query<&mut Style, With<UiUnitFrameRoot<T>>>,
) {
    if let Ok(mut style) = ui_query.get_single_mut() {
        match target_query.get_single() {
            Err(_) => style.display = Display::None,
            Ok(_) => style.display = Display::Flex,
        }
    }
}
