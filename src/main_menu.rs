use crate::app_state::AppState;
use crate::ui_style::{default_button_style, default_menu_style, default_text_style};
use bevy::app::AppExit;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Default, Resource)]
pub struct UiFont(pub Handle<Font>);

#[derive(Component)]
struct MainMenuUI;

#[derive(Component, Debug, PartialEq)]
pub enum MainMenuButton {
    Start,
    Options,
    Quit,
}
#[derive(Bundle)]
struct ButtonTarget {
    target: MainMenuButton,
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
                        app_state.set(AppState::LoadingGame);
                        info!("Start game");
                    }
                    MainMenuButton::Quit => {
                        exit.send(AppExit);
                        info!("Quit game");
                    }
                    MainMenuButton::Options => {
                        info!("Options");
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

fn setup_ui(mut commands: Commands, font: Res<UiFont>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 0,
            ..default()
        },
        ..default()
    });
    commands
        .spawn((
            MainMenuUI,
            NodeBundle {
                style: default_menu_style(),
                background_color: BackgroundColor(Color::GRAY),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonTarget {
                            target: MainMenuButton::Start,
                        },
                        TextBundle::from_section("Start Game", default_text_style(font.0.clone())),
                    ));
                });
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonTarget {
                            target: MainMenuButton::Options,
                        },
                        TextBundle::from_section("Options", default_text_style(font.0.clone())),
                    ));
                });
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonTarget {
                            target: MainMenuButton::Quit,
                        },
                        TextBundle::from_section("Quit", default_text_style(font.0.clone())),
                    ));
                });
        });
}

fn cleanup_ui(mut commands: Commands, mut query: Query<(Entity, &MainMenuUI)>) {
    for (e, _) in query.iter_mut() {
        commands.entity(e).despawn_recursive()
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFont>()
            .add_systems(OnEnter(AppState::MainMenu), setup_ui)
            .add_systems(OnExit(AppState::MainMenu), cleanup_ui)
            .add_systems(Update, button_system);
    }
}
