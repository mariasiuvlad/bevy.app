use bevy::prelude::*;

use super::super::components::{
    UiEnergyPercentage, UiEnergyValue, UiHealthPercentage, UiHealthValue, UiName, UiUnitFrameRoot,
};

use crate::{
    main_menu::UiFont,
    ui::nodes::{bar, container, text, unit_frame},
    world3d::{Player, PlayerTarget},
};

pub fn setup_unitframes(mut commands: Commands, ui_font: Res<UiFont>) {
    commands.spawn(container()).with_children(|parent| {
        parent
            .spawn((UiUnitFrameRoot(Player), unit_frame()))
            .with_children(|parent| {
                parent.spawn((UiName(Player), text(ui_font.0.clone())));
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
            .spawn((UiUnitFrameRoot(PlayerTarget), unit_frame()))
            .with_children(|parent| {
                parent.spawn((UiName(PlayerTarget), text(ui_font.0.clone())));
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
