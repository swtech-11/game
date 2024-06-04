use bevy::{prelude::*, window::PrimaryWindow};
use bevy_persistent::prelude::*;
use bevy_persistent_windows::prelude::*;

pub struct PersistentWindowPlugin;

impl Plugin for PersistentWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPlugin {
            primary_window: None,
            ..Default::default()
        });

        let state_directory = dirs::data_dir()
            .expect("failed to get the platforms data directory")
            .join("your-amazing-game")
            .join("state");

        app.world.spawn((
            PrimaryWindow,
            PersistentWindowBundle {
                window: Window {
                    title: "I am the primary window!".to_owned(),
                    ..Default::default()
                },
                state: Persistent::<WindowState>::builder()
                    .name("primary window state")
                    .format(StorageFormat::Toml)
                    .path(state_directory.join("primary-window.toml"))
                    .default(WindowState::windowed(1280, 720))
                    .revertible(true)
                    .revert_to_default_on_deserialization_errors(true)
                    .build()
                    .expect("failed to create the persistent primary window state"),
            },
        ));

        app.add_plugins(PersistentWindowsPlugin);
    }
}
