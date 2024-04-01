use bevy::{prelude::*, window::PresentMode};

pub fn get_window_config() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "I am a window!".into(),
            name: Some("bevy.app".into()),
            // resolution: (500., 300.).into(),
            // mode: bevy::window::WindowMode::BorderlessFullscreen,
            present_mode: PresentMode::AutoVsync,
            // // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            // prevent_default_event_handling: false,
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            // This will spawn an invisible window
            // The window will be made visible in the make_visible() system after 3 frames.
            // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
            visible: true,
            ..default()
        }),
        ..default()
    }
}
