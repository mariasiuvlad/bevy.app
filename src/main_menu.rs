use crate::app_state::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Debug, PartialEq)] // needed for comparison
pub enum MainMenuButton {
    Start,
    Options,
    Quit,
}
#[derive(Bundle)] // needed for comparison
struct ButtonTarget {
    target: MainMenuButton,
}

fn default_menu_style() -> Style {
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

fn default_button_style() -> Style {
    Style {
        width: Val::Px(240.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn default_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    }
}

fn default_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: default_button_style(),
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

fn button_system(
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    button_target_query: Query<&MainMenuButton>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let target = button_target_query.get(children[0]).unwrap();
                match *target {
                    MainMenuButton::Start => {
                        app_state.set(AppState::InGame);
                        println!("Start game");
                    }
                    MainMenuButton::Quit => {
                        exit.send(AppExit);
                        println!("Quit game");
                    }
                    MainMenuButton::Options => {
                        println!("Options");
                    }
                }
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub struct MainMenuPlugin;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: default_menu_style(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonTarget {
                            target: MainMenuButton::Start,
                        },
                        TextBundle::from_section(
                            "Start Game",
                            default_text_style(asset_server.load("fonts/JetBrainsMono-Medium.ttf")),
                        ),
                    ));
                });
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonTarget {
                            target: MainMenuButton::Options,
                        },
                        TextBundle::from_section(
                            "Options",
                            default_text_style(asset_server.load("fonts/JetBrainsMono-Medium.ttf")),
                        ),
                    ));
                });
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonTarget {
                            target: MainMenuButton::Quit,
                        },
                        TextBundle::from_section(
                            "Quit",
                            default_text_style(asset_server.load("fonts/JetBrainsMono-Medium.ttf")),
                        ),
                    ));
                });
        });
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_ui)
            .add_systems(Update, button_system);
    }
}
