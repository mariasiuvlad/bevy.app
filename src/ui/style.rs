use bevy::prelude::*;

pub fn default_menu_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        row_gap: Val::Px(16.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

pub fn default_button_style() -> Style {
    Style {
        width: Val::Px(240.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn default_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    }
}

pub fn player_ui_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: 14.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    }
}

pub fn nameplate_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: 28.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    }
}
