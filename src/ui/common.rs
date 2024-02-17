use bevy::prelude::*;

use crate::ui_style::player_ui_text_style;

pub fn container() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
        ..default()
    }
}

pub fn unit_frame() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Px(16.)),
            width: Val::Px(200.),
            height: Val::Px(40.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        border_color: BorderColor(Color::WHITE),
        ..default()
    }
}

pub fn bar(color: Color) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            flex_grow: 1.,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: BackgroundColor(color),
        ..default()
    }
}

pub fn text(font: Handle<Font>) -> TextBundle {
    TextBundle::from_sections([TextSection::new("", player_ui_text_style(font))])
}
