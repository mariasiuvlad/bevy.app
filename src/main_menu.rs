use crate::app_state::AppState;
use crate::components::cleanup::{cleanup, CleanupMainMenuClose};
use crate::ui_style::{default_button_style, default_menu_style, default_text_style};
use bevy::app::AppExit;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Event, Debug, Clone, Copy)]
pub enum ButtonPressedEvent {
    StartGame,
    Quit,
    Options,
}

#[derive(Default, Resource)]
pub struct UiFont(pub Handle<Font>);

#[derive(Component, Debug)]
struct ButtonAction(ButtonPressedEvent);

fn default_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: default_button_style(),
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

fn button_press_controller(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut ev_button_pressed: EventWriter<ButtonPressedEvent>,
    button_target_query: Query<&ButtonAction>,
) {
    for (interaction, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(target) = button_target_query.get(children[0]) {
                    ev_button_pressed.send(target.0);
                }
            }
            _ => {}
        }
    }
}

fn button_feedback_controller(
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

fn handle_button_pressed_event(
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    mut ev_button_pressed: EventReader<ButtonPressedEvent>,
) {
    for ev in ev_button_pressed.read() {
        info!("{:?}", ev);
        match ev {
            &ButtonPressedEvent::StartGame => {
                app_state.set(AppState::Game);
            }
            &ButtonPressedEvent::Quit => {
                exit.send(AppExit);
            }
            &ButtonPressedEvent::Options => {
                todo!()
            }
        }
    }
}

fn setup_ui(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            CleanupMainMenuClose,
            NodeBundle {
                style: default_menu_style(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonAction(ButtonPressedEvent::StartGame),
                        TextBundle::from_section("Start Game", default_text_style(font.0.clone())),
                    ));
                });
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonAction(ButtonPressedEvent::Options),
                        TextBundle::from_section("Options", default_text_style(font.0.clone())),
                    ));
                });
            parent
                .spawn(default_button_bundle())
                .with_children(|parent| {
                    parent.spawn((
                        ButtonAction(ButtonPressedEvent::Quit),
                        TextBundle::from_section("Quit", default_text_style(font.0.clone())),
                    ));
                });
        });
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFont>()
            .add_event::<ButtonPressedEvent>()
            .add_systems(OnEnter(AppState::MainMenu), setup_ui)
            .add_systems(OnExit(AppState::MainMenu), cleanup::<CleanupMainMenuClose>)
            .add_systems(
                Update,
                (
                    button_press_controller,
                    button_feedback_controller,
                    handle_button_pressed_event,
                ),
            );
    }
}
