use crate::config::{CELL_SIZE, GRID_SIZE};
use bevy_persistent::prelude::*;
use bevy_persistent_windows::prelude::*;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
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
                    .path("persistent/primary_window_state.toml")
                    .default(WindowState::windowed(1280, 720))
                    .revertible(true)
                    .revert_to_default_on_deserialization_errors(true)
                    .build()
                    .expect("failed to create the persistent primary window state"),
            },
        ));

        app.add_plugins((
            ScreenFrameDiagnosticsPlugin,
            ScreenDiagnosticsPlugin::default(),
            PersistentWindowsPlugin,
        ))
        .add_systems(Startup, config_gizmos)
        .add_systems(Update, draw_grid);
    }
}

fn config_gizmos(mut gizmo_store: ResMut<GizmoConfigStore>) {
    let (config, _) = gizmo_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 0.1;
}

fn draw_grid(mut gizmos: Gizmos) {
    let length = GRID_SIZE as f32 * CELL_SIZE;
    for anchor_1 in 0..GRID_SIZE + 1 {
        let anchor_2 = anchor_1 as f32 * CELL_SIZE;
        gizmos.line_2d(
            Vec2::new(anchor_2, 0.0),
            Vec2::new(anchor_2 as f32, length),
            Color::GRAY,
        );
        gizmos.line_2d(
            Vec2::new(0.0, anchor_2),
            Vec2::new(length, anchor_2 as f32),
            Color::GRAY,
        );
    }
}
