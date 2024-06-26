use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::game_logic::ConfigRes;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, config: Res<ConfigRes>) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.05,
                ..default()
            },
            transform: Transform::from_xyz(
                (config.bounds.0.x + config.bounds.1.x) / 2.0,
                (config.bounds.0.y + config.bounds.1.y) / 2.0,
                0.0,
            ),
            ..default()
        })
        .insert(PanCam::default());
}
