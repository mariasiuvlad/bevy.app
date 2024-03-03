use crate::app_state::AppState;
use crate::components::cleanup::{cleanup, MainMenuClose};
use crate::ui_style::{default_button_style, default_menu_style, default_text_style};
use bevy::app::AppExit;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Event, Debug)]
pub struct ButtonPressed(pub MainMenuButton);

#[derive(Default, Resource)]
pub struct UiFont(pub Handle<Font>);

#[derive(Component, Debug, PartialEq, Clone, Copy)]
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

fn handle_button_press(
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    mut ev_button_pressed: EventReader<ButtonPressed>,
) {
    for ev in ev_button_pressed.read() {
        match ev.0 {
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
    }
}

fn button_press(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut ev_button_pressed: EventWriter<ButtonPressed>,
    button_target_query: Query<&MainMenuButton>,
) {
    for (interaction, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let target: &MainMenuButton = button_target_query.get(children[0]).unwrap();
                ev_button_pressed.send(ButtonPressed(*target));
            }
            _ => {}
        }
    }
}

fn button_feedback(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
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
    commands
        .spawn((
            MainMenuClose,
            NodeBundle {
                style: default_menu_style(),
                // background_color: BackgroundColor(Color::BLACK),
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

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFont>()
            .add_event::<ButtonPressed>()
            .add_systems(OnEnter(AppState::MainMenu), setup_ui)
            .add_systems(OnExit(AppState::MainMenu), cleanup::<MainMenuClose>)
            .add_systems(Update, (button_press, button_feedback, handle_button_press));
    }
}
